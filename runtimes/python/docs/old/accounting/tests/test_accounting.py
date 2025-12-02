# test_accounting.py (fixed syntax, inconsistencies, and coverage gaps)
import unittest
from unittest.mock import patch, MagicMock, mock_open
from decimal import Decimal, getcontext
import decimal
from datetime import datetime, date
from accounting.models import Account, AccountType, Context, Transaction, TransactionEntry, Ledger
from accounting.config import AccountingConfig
from accounting.service import AccountingService
from accounting.persistence import save_ledger, load_ledger
from accounting.integrations.firefly import FireflyIntegrator
import os
import tempfile
import subprocess
import sys
import json
from io import StringIO

class TestConfig(unittest.TestCase):
    def test_default_config(self):
        config = AccountingConfig()
        self.assertEqual(config.precision, 4)
        self.assertEqual(config.rounding, 'ROUND_HALF_UP')
        self.assertTrue(config.validate_on_add)
        self.assertEqual(config.log_level, 'INFO')
        self.assertEqual(config.persistence_format, 'parquet')
        self.assertFalse(config.log_transactions)

    def test_custom_config(self):
        config = AccountingConfig(precision=2, rounding='ROUND_HALF_EVEN', validate_on_add=False, log_level='DEBUG', persistence_format='feather', log_transactions=True)
        self.assertEqual(config.precision, 2)
        self.assertEqual(config.rounding, 'ROUND_HALF_EVEN')
        self.assertFalse(config.validate_on_add)
        self.assertEqual(config.log_level, 'DEBUG')
        self.assertEqual(config.persistence_format, 'feather')
        self.assertTrue(config.log_transactions)

    def test_invalid_rounding(self):
        with self.assertRaises(ValueError):
            AccountingConfig(rounding='INVALID')

    def test_env_file_loading(self):
        # Mock env file
        mock_env = {'ACCOUNTING_PRECISION': '6', 'ACCOUNTING_ROUNDING': 'ROUND_DOWN'}
        with patch('os.getenv', side_effect=lambda k, d=None: mock_env.get(k, d)):
            config = AccountingConfig()
            self.assertEqual(config.precision, 6)
            self.assertEqual(config.rounding, 'ROUND_DOWN')

class TestModels(unittest.TestCase):
    def test_account_creation(self):
        account = Account(name="Cash", type=AccountType.ASSET, context=Context.PERSONAL)
        self.assertEqual(account.name, "Cash")
        self.assertEqual(account.balance, Decimal('0.00'))
        self.assertIsNotNone(account.id)

    def test_account_description_optional(self):
        account = Account(name="Cash", type=AccountType.ASSET, context=Context.PERSONAL, description="Liquid assets")
        self.assertEqual(account.description, "Liquid assets")

    def test_account_invalid_type(self):
        with self.assertRaises(ValueError):
            Account(name="Cash", type="INVALID", context=Context.PERSONAL)

    def test_transaction_entry_positive_amount(self):
        with self.assertRaises(ValueError):
            TransactionEntry(account_id="1", amount=Decimal('-1'), is_debit=True)
        entry = TransactionEntry(account_id="1", amount=Decimal('0'), is_debit=True)
        self.assertEqual(entry.amount, Decimal('0'))

    def test_transaction_balanced(self):
        entries = [
            TransactionEntry(account_id="1", amount=Decimal('100.1234'), is_debit=True),
            TransactionEntry(account_id="2", amount=Decimal('100.1234'), is_debit=False)
        ]
        tx = Transaction(description="Test", entries=entries, context=Context.PERSONAL)
        self.assertIsNotNone(tx.id)
        self.assertIsInstance(tx.date, datetime)

    def test_transaction_unbalanced(self):
        entries = [
            TransactionEntry(account_id="1", amount=Decimal('100'), is_debit=True),
            TransactionEntry(account_id="2", amount=Decimal('50'), is_debit=False)
        ]
        with self.assertRaises(ValueError):
            Transaction(description="Test", entries=entries, context=Context.PERSONAL)

    def test_transaction_many_entries_balanced(self):
        entries = [TransactionEntry(account_id=f"id{i}", amount=Decimal('10'), is_debit=(i%2==0)) for i in range(10)]
        total_debit = sum(e.amount for e in entries if e.is_debit)
        total_credit = sum(e.amount for e in entries if not e.is_debit)
        self.assertEqual(total_debit, total_credit)  # 5 debits, 5 credits of 10 each
        tx = Transaction(description="Multi", entries=entries, context=Context.PERSONAL)
        self.assertIsNotNone(tx)

    def test_transaction_empty_entries(self):
        with self.assertRaises(ValueError):
            Transaction(description="Test", entries=[], context=Context.PERSONAL)

    def test_ledger_creation(self):
        ledger = Ledger(name="Test", context=Context.PERSONAL)
        self.assertEqual(ledger.name, "Test")
        self.assertEqual(ledger.context, Context.PERSONAL)
        self.assertEqual(len(ledger.accounts), 0)
        self.assertEqual(len(ledger.transactions), 0)

class TestService(unittest.TestCase):
    def setUp(self):
        self.config = AccountingConfig(precision=10, validate_on_add=True, log_transactions=False, persistence_format='parquet')
        self.service = AccountingService(self.config)
        self.ledger = Ledger(name="Test", context=Context.PERSONAL)
        self.cash = Account(name="Cash", type=AccountType.ASSET, context=Context.PERSONAL)
        self.expense = Account(name="Expense", type=AccountType.EXPENSE, context=Context.PERSONAL)
        self.revenue = Account(name="Revenue", type=AccountType.REVENUE, context=Context.PERSONAL)
        self.service.add_account(self.ledger, self.cash)
        self.service.add_account(self.ledger, self.expense)
        self.service.add_account(self.ledger, self.revenue)

    def test_service_init_default(self):
        default_config = AccountingConfig()
        service = AccountingService(default_config)
        self.assertEqual(getcontext().prec, 4)
        self.assertEqual(getcontext().rounding, getattr(decimal, 'ROUND_HALF_UP'))

    def test_service_init_custom(self):
        custom_config = AccountingConfig(precision=2, rounding='ROUND_DOWN')
        service = AccountingService(custom_config)
        self.assertEqual(getcontext().prec, 2)
        self.assertEqual(getcontext().rounding, getattr(decimal, 'ROUND_DOWN'))

    def test_add_account(self):
        new_acc = Account(name="New", type=AccountType.LIABILITY, context=Context.PERSONAL)
        self.service.add_account(self.ledger, new_acc)
        self.assertIn(new_acc, self.ledger.accounts)
        self.assertEqual(len(self.ledger.accounts), 4)

    def test_add_duplicate_account(self):
        with self.assertRaises(ValueError):
            self.service.add_account(self.ledger, self.cash)

    def test_add_account_without_validation(self):
        config_no_val = AccountingConfig(validate_on_add=False, precision=10)
        service_no_val = AccountingService(config_no_val)
        ledger_dup = Ledger(name="DupTest", context=Context.PERSONAL)
        service_no_val.add_account(ledger_dup, self.cash)
        service_no_val.add_account(ledger_dup, self.cash)
        self.assertEqual(len(ledger_dup.accounts), 2)

    def test_add_transaction(self):
        entry1 = TransactionEntry(account_id=self.expense.id, amount=Decimal('100.00'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.cash.id, amount=Decimal('100.00'), is_debit=False)
        entries = [entry1, entry2]
        tx = Transaction(description="Test", entries=entries, context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx)
        self.assertEqual(self.expense.balance, Decimal('100.00'))
        self.assertEqual(self.cash.balance, Decimal('-100.00'))

    def test_add_multiple_transactions(self):
        entry1 = TransactionEntry(account_id=self.expense.id, amount=Decimal('100'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.cash.id, amount=Decimal('100'), is_debit=False)
        tx1 = Transaction(description="Tx1", entries=[entry1, entry2], context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx1)
        entry3 = TransactionEntry(account_id=self.cash.id, amount=Decimal('200'), is_debit=True)
        entry4 = TransactionEntry(account_id=self.revenue.id, amount=Decimal('200'), is_debit=False)
        tx2 = Transaction(description="Tx2", entries=[entry3, entry4], context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx2)
        self.assertEqual(self.expense.balance, Decimal('100.00'))
        self.assertEqual(self.cash.balance, Decimal('100.00'))
        self.assertEqual(self.revenue.balance, Decimal('-200.00'))

    def test_add_transaction_context_mismatch(self):
        entry1 = TransactionEntry(account_id=self.cash.id, amount=Decimal('100'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.expense.id, amount=Decimal('100'), is_debit=False)
        entries = [entry1, entry2]
        tx = Transaction(description="Test", entries=entries, context=Context.BUSINESS)
        with self.assertRaises(ValueError):
            self.service.add_transaction(self.ledger, tx)

    def test_add_transaction_account_not_found(self):
        entry1 = TransactionEntry(account_id="missing", amount=Decimal('100'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.cash.id, amount=Decimal('100'), is_debit=False)
        entries = [entry1, entry2]
        tx = Transaction(description="Test", entries=entries, context=Context.PERSONAL)
        with self.assertRaises(ValueError):
            self.service.add_transaction(self.ledger, tx)

    def test_get_account_balance(self):
        entry1 = TransactionEntry(account_id=self.cash.id, amount=Decimal('200.5678'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.expense.id, amount=Decimal('200.5678'), is_debit=False)
        entries = [entry1, entry2]
        tx = Transaction(description="Test", entries=entries, context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx)
        self.assertEqual(self.service.get_account_balance(self.ledger, self.cash.id), Decimal('200.5678'))

    def test_get_account_balance_not_found(self):
        with self.assertRaises(ValueError):
            self.service.get_account_balance(self.ledger, "nonexistent")

    def test_generate_trial_balance(self):
        entry1 = TransactionEntry(account_id=self.cash.id, amount=Decimal('300.9999'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.expense.id, amount=Decimal('300.9999'), is_debit=False)
        entries = [entry1, entry2]
        tx = Transaction(description="Test", entries=entries, context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx)
        trial = self.service.generate_trial_balance(self.ledger)
        self.assertEqual(trial['Cash'], '300.9999')
        self.assertEqual(trial['Expense'], '-300.9999')
        self.assertIn('Revenue', trial)
        self.assertEqual(trial['Revenue'], '0.00')

    def test_logging_on_add_transaction(self):
        config_log = AccountingConfig(log_transactions=True, log_level='INFO')
        service_log = AccountingService(config_log)
        with patch('logging.getLogger') as mock_logger:
            mock_logger_instance = MagicMock()
            mock_logger.return_value = mock_logger_instance
            entry1 = TransactionEntry(account_id=self.expense.id, amount=Decimal('100'), is_debit=True)
            entry2 = TransactionEntry(account_id=self.cash.id, amount=Decimal('100'), is_debit=False)
            tx = Transaction(description="LogTest", entries=[entry1, entry2], context=Context.PERSONAL)
            service_log.add_transaction(self.ledger, tx)
            mock_logger_instance.info.assert_called_with("Added transaction: LogTest")

    @patch('accounting.persistence.save_ledger')
    def test_save(self, mock_save):
        self.service.save(self.ledger, '/tmp/test')
        mock_save.assert_called_once_with(self.ledger, '/tmp/test', 'parquet')

    @patch('accounting.persistence.load_ledger')
    def test_load(self, mock_load):
        mock_ledger = MagicMock()
        mock_load.return_value = mock_ledger
        loaded = self.service.load('/tmp/test')
        mock_load.assert_called_once_with('/tmp/test', 'parquet')
        self.assertEqual(loaded, mock_ledger)

    def test_save_and_load_parquet(self):
        tx1 = Transaction(description="Salary", entries=[
            TransactionEntry(account_id=self.cash.id, amount=Decimal('5000'), is_debit=True),
            TransactionEntry(account_id=self.expense.id, amount=Decimal('5000'), is_debit=False)
        ], context=Context.PERSONAL)
        tx2 = Transaction(description="Bonus", entries=[
            TransactionEntry(account_id=self.cash.id, amount=Decimal('1000'), is_debit=True),
            TransactionEntry(account_id=self.revenue.id, amount=Decimal('1000'), is_debit=False)
        ], context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx1)
        self.service.add_transaction(self.ledger, tx2)

        with tempfile.TemporaryDirectory() as tmpdir:
            path = os.path.join(tmpdir, 'test_ledger')
            self.service.save(self.ledger, path)
            loaded_ledger = self.service.load(path)
            self.assertEqual(len(loaded_ledger.accounts), 3)
            self.assertEqual(len(loaded_ledger.transactions), 2)
            self.assertEqual(self.service.get_account_balance(loaded_ledger, self.cash.id), Decimal('6000'))
            self.assertEqual(self.service.get_account_balance(loaded_ledger, self.expense.id), Decimal('-5000'))
            self.assertEqual(self.service.get_account_balance(loaded_ledger, self.revenue.id), Decimal('-1000'))

    def test_save_and_load_feather(self):
        config_feather = AccountingConfig(persistence_format='feather')
        service_feather = AccountingService(config_feather)
        # Similar setup as above
        tx1 = Transaction(description="Salary", entries=[
            TransactionEntry(account_id=self.cash.id, amount=Decimal('5000'), is_debit=True),
            TransactionEntry(account_id=self.expense.id, amount=Decimal('5000'), is_debit=False)
        ], context=Context.PERSONAL)
        service_feather.add_transaction(self.ledger, tx1)

        with tempfile.TemporaryDirectory() as tmpdir:
            path = os.path.join(tmpdir, 'test_ledger_feather')
            service_feather.save(self.ledger, path)
            loaded_ledger = service_feather.load(path)
            self.assertEqual(len(loaded_ledger.transactions), 1)
            self.assertEqual(service_feather.get_account_balance(loaded_ledger, self.cash.id), Decimal('-5000'))

    @patch('accounting.persistence.load_ledger')
    def test_load_missing_file(self, mock_load):
        mock_load.side_effect = FileNotFoundError
        with self.assertRaises(FileNotFoundError):
            self.service.load('/nonexistent')

    def test_rounding_in_balance(self):
        config_round = AccountingConfig(precision=2, rounding='ROUND_HALF_UP')
        service_round = AccountingService(config_round)
        entry1 = TransactionEntry(account_id=self.cash.id, amount=Decimal('1.235'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.expense.id, amount=Decimal('1.235'), is_debit=False)
        tx = Transaction(description="RoundTest", entries=[entry1, entry2], context=Context.PERSONAL)
        service_round.add_transaction(self.ledger, tx)
        self.assertEqual(service_round.get_account_balance(self.ledger, self.cash.id), Decimal('1.24'))

    @patch('accounting.integrations.firefly.Configuration')
    @patch('accounting.integrations.firefly.AccountsApi')
    @patch('accounting.integrations.firefly.TransactionsApi')
    def test_firefly_export_mock(self, mock_tx_api, mock_acc_api, mock_config_class):
        tx = Transaction(description="Test", entries=[
            TransactionEntry(account_id=self.cash.id, amount=Decimal('100'), is_debit=True),
            TransactionEntry(account_id=self.expense.id, amount=Decimal('100'), is_debit=False)
        ], context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx)
        mock_accounts_resp = MagicMock()
        mock_accounts_resp.data = []
        mock_acc_api.return_value.list_accounts.return_value = mock_accounts_resp
        mock_created = MagicMock()
        mock_created.data = MagicMock(id=1)
        mock_acc_api.return_value.store_account.return_value = mock_created
        mock_tx_api.return_value.store_transaction.return_value = None
        mock_config = MagicMock()
        mock_config.proxy = None
        mock_config_class.return_value = mock_config

        integrator = FireflyIntegrator(self.config, "http://test", "token")
        integrator.export_ledger(self.service, self.ledger)

        self.assertEqual(len(mock_acc_api.return_value.store_account.call_args_list), 3)
        mock_tx_api.return_value.store_transaction.assert_called_once()

class TestPersistence(unittest.TestCase):
    def setUp(self):
        self.ledger = Ledger(name="PersistTest", context=Context.PERSONAL)
        self.cash = Account(name="Cash", type=AccountType.ASSET, context=Context.PERSONAL)
        self.dummy = Account(name="Dummy", type=AccountType.ASSET, context=Context.PERSONAL)
        self.ledger.accounts.append(self.cash)
        self.ledger.accounts.append(self.dummy)
        tx = Transaction(description="Test", entries=[TransactionEntry(self.cash.id, Decimal('100'), True), TransactionEntry(self.dummy.id, Decimal('100'), False)], context=Context.PERSONAL)
        self.ledger.transactions.append(tx)

    @patch('polars.DataFrame.write_parquet')
    @patch('builtins.open', new_callable=mock_open)
    def test_save_ledger_parquet(self, mock_file, mock_write):
        save_ledger(self.ledger, '/tmp/persist', 'parquet')
        mock_write.assert_any_call('/tmp/persist/accounts.parquet')
        mock_write.assert_any_call('/tmp/persist/transactions.parquet')
        mock_write.assert_any_call('/tmp/persist/entries.parquet')
        mock_file.assert_called_with('/tmp/persist/metadata.json', 'w')

    @patch('polars.DataFrame.write_ipc')
    @patch('builtins.open', new_callable=mock_open)
    def test_save_ledger_feather(self, mock_file, mock_write):
        save_ledger(self.ledger, '/tmp/persist', 'feather')
        mock_write.assert_any_call('/tmp/persist/accounts.feather')
        mock_file.assert_called_with('/tmp/persist/metadata.json', 'w')

    @patch('polars.read_parquet')
    @patch('builtins.open', new_callable=mock_open, read_data='{"id":"1","name":"PersistTest","context":"personal"}')
    def test_load_ledger_parquet(self, mock_file, mock_read):
        mock_df_acc = MagicMock()
        mock_df_acc.iter_rows.return_value = [
            {'id':'a1','name':'Cash','type':'asset','context':'personal','balance':'0','description':None},
            {'id':'a2','name':'Dummy','type':'asset','context':'personal','balance':'0','description':None}
        ]
        mock_df_trans = MagicMock()
        mock_df_trans.iter_rows.return_value = [{'id':'t1','date':'2023-01-01T00:00:00','description':'Test','context':'personal'}]
        mock_df_entries = MagicMock()
        mock_df_entries.iter_rows.return_value = [
            {'transaction_id':'t1','account_id':'a1','amount':'100','is_debit':True},
            {'transaction_id':'t1','account_id':'a2','amount':'100','is_debit':False}
        ]
        mock_read.side_effect = [mock_df_acc, mock_df_trans, mock_df_entries]
        loaded = load_ledger('/tmp/persist', 'parquet')
        self.assertEqual(loaded.name, 'PersistTest')
        self.assertEqual(len(loaded.accounts), 2)
        self.assertEqual(len(loaded.transactions), 1)
        self.assertEqual(loaded.accounts[0].balance, Decimal('100'))

    @patch('polars.read_ipc')
    @patch('builtins.open', new_callable=mock_open, read_data='{"id":"1","name":"PersistTest","context":"personal"}')
    def test_load_ledger_feather(self, mock_file, mock_read):
        mock_df_acc = MagicMock()
        mock_df_acc.iter_rows.return_value = [
            {'id':'a1','name':'Cash','type':'asset','context':'personal','balance':'0','description':None},
            {'id':'a2','name':'Dummy','type':'asset','context':'personal','balance':'0','description':None}
        ]
        mock_df_trans = MagicMock()
        mock_df_trans.iter_rows.return_value = [{'id':'t1','date':'2023-01-01T00:00:00','description':'Test','context':'personal'}]
        mock_df_entries = MagicMock()
        mock_df_entries.iter_rows.return_value = [
            {'transaction_id':'t1','account_id':'a1','amount':'100','is_debit':True},
            {'transaction_id':'t1','account_id':'a2','amount':'100','is_debit':False}
        ]
        mock_read.side_effect = [mock_df_acc, mock_df_trans, mock_df_entries]
        loaded = load_ledger('/tmp/persist', 'feather')
        self.assertEqual(loaded.name, 'PersistTest')
        self.assertEqual(len(loaded.accounts), 2)
        self.assertEqual(loaded.accounts[0].balance, Decimal('100'))

    @patch('builtins.open')
    def test_load_missing_metadata(self, mock_open):
        mock_open.side_effect = FileNotFoundError
        with self.assertRaises(FileNotFoundError):
            load_ledger('/nonexistent')

class TestFireflyMultiEntry(unittest.TestCase):
    def setUp(self):
        self.config = AccountingConfig()
        self.service = AccountingService(self.config)
        self.ledger = Ledger(name="Test", context=Context.PERSONAL)
        self.cash = Account(name="Cash", type=AccountType.ASSET, context=Context.PERSONAL)
        self.expense1 = Account(name="Expense1", type=AccountType.EXPENSE, context=Context.PERSONAL)
        self.expense2 = Account(name="Expense2", type=AccountType.EXPENSE, context=Context.PERSONAL)
        self.revenue = Account(name="Revenue", type=AccountType.REVENUE, context=Context.PERSONAL)
        self.service.add_account(self.ledger, self.cash)
        self.service.add_account(self.ledger, self.expense1)
        self.service.add_account(self.ledger, self.expense2)
        self.service.add_account(self.ledger, self.revenue)

    @patch('accounting.integrations.firefly.Configuration')
    @patch('accounting.integrations.firefly.AccountsApi')
    @patch('accounting.integrations.firefly.TransactionsApi')
    def test_firefly_export_multi_credit_single_debit(self, mock_tx_api, mock_acc_api, mock_config_class):
        entry1 = TransactionEntry(account_id=self.cash.id, amount=Decimal('300'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.expense1.id, amount=Decimal('100'), is_debit=False)
        entry3 = TransactionEntry(account_id=self.expense2.id, amount=Decimal('200'), is_debit=False)
        tx = Transaction(description="Multi Credit", entries=[entry1, entry2, entry3], context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx)

        mock_accounts_resp = MagicMock()
        mock_accounts_resp.data = []
        mock_acc_api.return_value.list_accounts.return_value = mock_accounts_resp

        mock_created = MagicMock()
        mock_created.data = MagicMock(id=1)
        mock_acc_api.return_value.store_account.return_value = mock_created

        mock_tx_api.return_value.store_transaction.return_value = None
        mock_config = MagicMock()
        mock_config.proxy = None
        mock_config_class.return_value = mock_config

        integrator = FireflyIntegrator(self.config, "http://test", "token")
        integrator.export_ledger(self.service, self.ledger)
        mock_tx_api.return_value.store_transaction.assert_called_once()

    @patch('accounting.integrations.firefly.Configuration')
    @patch('accounting.integrations.firefly.AccountsApi')
    @patch('accounting.integrations.firefly.TransactionsApi')
    def test_firefly_export_single_credit_multi_debit(self, mock_tx_api, mock_acc_api, mock_config_class):
        entry1 = TransactionEntry(account_id=self.expense1.id, amount=Decimal('100'), is_debit=True)
        entry2 = TransactionEntry(account_id=self.expense2.id, amount=Decimal('200'), is_debit=True)
        entry3 = TransactionEntry(account_id=self.cash.id, amount=Decimal('300'), is_debit=False)
        tx = Transaction(description="Multi Debit", entries=[entry1, entry2, entry3], context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx)

        mock_accounts_resp = MagicMock()
        mock_accounts_resp.data = []
        mock_acc_api.return_value.list_accounts.return_value = mock_accounts_resp
        mock_created = MagicMock()
        mock_created.data = MagicMock(id=1)
        mock_acc_api.return_value.store_account.return_value = mock_created
        mock_config = MagicMock()
        mock_config.proxy = None
        mock_config_class.return_value = mock_config

        integrator = FireflyIntegrator(self.config, "http://test", "token")
        integrator.export_ledger(self.service, self.ledger)
        mock_tx_api.return_value.store_transaction.assert_called_once()

    @patch('accounting.integrations.firefly.Configuration')
    @patch('accounting.integrations.firefly.AccountsApi')
    def test_firefly_account_exists(self, mock_acc_api, mock_config_class):
        self.service.add_account(self.ledger, self.cash)
        mock_accounts_resp = MagicMock()
        mock_acc_data = MagicMock()
        mock_acc_data.attributes = MagicMock(name='Cash', account_type='asset')
        mock_acc_data.id = '1'
        mock_accounts_resp.data = [mock_acc_data]
        mock_acc_api.return_value.list_accounts.return_value = mock_accounts_resp
        mock_config = MagicMock()
        mock_config.proxy = None
        mock_config_class.return_value = mock_config

        integrator = FireflyIntegrator(self.config, "http://test", "token")
        acc_id = integrator._create_or_get_account(self.cash)
        self.assertEqual(acc_id, 1)
        mock_acc_api.return_value.store_account.assert_not_called()

    @patch('accounting.integrations.firefly.Configuration')
    @patch('accounting.integrations.firefly.AccountsApi')
    @patch('accounting.integrations.firefly.TransactionsApi')
    def test_firefly_export_no_transactions(self, mock_tx_api, mock_acc_api, mock_config_class):
        mock_accounts_resp = MagicMock()
        mock_accounts_resp.data = []
        mock_acc_api.return_value.list_accounts.return_value = mock_accounts_resp
        mock_config = MagicMock()
        mock_config.proxy = None
        mock_config_class.return_value = mock_config

        integrator = FireflyIntegrator(self.config, "http://test", "token")
        integrator.export_ledger(self.service, self.ledger)  # Empty tx
        mock_tx_api.return_value.store_transaction.assert_not_called()

    @patch('accounting.integrations.firefly.Configuration')
    @patch('accounting.integrations.firefly.AccountsApi')
    @patch('accounting.integrations.firefly.TransactionsApi')
    def test_firefly_export_unbalanced_transaction(self, mock_tx_api, mock_acc_api, mock_config_class):
        # Unbalanced tx
        entry1 = TransactionEntry(account_id=self.cash.id, amount=Decimal('100'), is_debit=True)
        tx = Transaction(description="Unbalanced", entries=[entry1], context=Context.PERSONAL)
        self.service.add_transaction(self.ledger, tx)  # But model prevents unbalanced, so mock or adjust
        # Since model validator prevents, test raises
        with self.assertRaises(ValueError):
            integrator = FireflyIntegrator(self.config, "http://test", "token")
            integrator.export_ledger(self.service, self.ledger)
        mock_tx_api.return_value.store_transaction.assert_not_called()

class TestCLIIntegration(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.app_path = 'src/accounting/app.py'
        cls.env = os.environ.copy()

    def test_cli_create_success(self):
        result = subprocess.run([sys.executable, self.app_path, 'create', 'TestLedger', '--context', 'personal'], 
                                capture_output=True, text=True, env=self.env)
        self.assertEqual(result.returncode, 0)
        self.assertIn('Created ledger', result.stdout)

    def test_cli_add_account_without_ledger(self):
        result = subprocess.run([sys.executable, self.app_path, 'add-account', 'Cash', '--type', 'asset'], 
                                capture_output=True, text=True, env=self.env)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('No ledger', result.stderr)

    def test_cli_balance_invalid_account(self):
        subprocess.run([sys.executable, self.app_path, 'create', 'Temp'], capture_output=True, env=self.env)
        result = subprocess.run([sys.executable, self.app_path, 'balance', 'invalid'], 
                                capture_output=True, text=True, env=self.env)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Account not found', result.stdout)

    def test_cli_trial_balance_no_ledger(self):
        result = subprocess.run([sys.executable, self.app_path, 'trial-balance'], 
                                capture_output=True, text=True, env=self.env)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('No ledger', result.stderr)

    def test_cli_help(self):
        result = subprocess.run([sys.executable, self.app_path, '--help'], capture_output=True, text=True, env=self.env)
        self.assertEqual(result.returncode, 0)
        self.assertIn('Commands:', result.stdout)

    def test_cli_create_invalid_context(self):
        result = subprocess.run([sys.executable, self.app_path, 'create', 'Test', '--context', 'invalid'], 
                                capture_output=True, text=True, env=self.env)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Invalid', result.stderr)  # Enum error

    def test_cli_add_account_invalid_type(self):
        subprocess.run([sys.executable, self.app_path, 'create', 'Temp'], capture_output=True, env=self.env)
        result = subprocess.run([sys.executable, self.app_path, 'add-account', 'Cash', '--type', 'invalid'], 
                                capture_output=True, text=True, env=self.env)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Invalid', result.stderr)

    def test_cli_load_nonexistent(self):
        result = subprocess.run([sys.executable, self.app_path, 'load', '/nonexistent'], 
                                capture_output=True, text=True, env=self.env)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Error', result.stderr)

    def test_cli_export_firefly_missing_args(self):
        result = subprocess.run([sys.executable, self.app_path, 'export-firefly'], 
                                capture_output=True, text=True, env=self.env)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Missing', result.stderr)

if __name__ == '__main__':
    unittest.main(verbosity=2)