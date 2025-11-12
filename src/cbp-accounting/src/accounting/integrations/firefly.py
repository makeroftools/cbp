# firefly.py (updated for correct split handling)
from firefly_iii_client import ApiClient, Configuration, AccountsApi, TransactionsApi
from accounting.service import AccountingService
from accounting.models import Ledger, AccountType
from accounting.config import AccountingConfig
from decimal import Decimal
from typing import Dict

class FireflyIntegrator:
    def __init__(self, config: AccountingConfig, base_url: str, token: str):
        self.config = config
        api_config = Configuration(host=base_url)
        api_config.access_token = token
        self.api_client = ApiClient(configuration=api_config)
        self.accounts_api = AccountsApi(self.api_client)
        self.transactions_api = TransactionsApi(self.api_client)
        self.account_map: Dict[str, int] = {}

    def _map_account_type(self, acc_type: AccountType) -> str:
        mapping = {
            AccountType.ASSET: 'asset',
            AccountType.LIABILITY: 'liability',
            AccountType.EQUITY: 'liability',
            AccountType.REVENUE: 'revenue',
            AccountType.EXPENSE: 'expense'
        }
        return mapping.get(acc_type, 'asset')

    def _create_or_get_account(self, local_acc):
        if local_acc.id in self.account_map:
            return self.account_map[local_acc.id]
        
        accounts_resp = self.accounts_api.list_accounts(limit=100)
        for acc_data in accounts_resp.data or []:
            if (acc_data.attributes.name == local_acc.name and 
                acc_data.attributes.account_type == self._map_account_type(local_acc.type)):
                firefly_id = int(acc_data.id)
                self.account_map[local_acc.id] = firefly_id
                return firefly_id
        
        new_acc = {
            'name': local_acc.name,
            'account_type': self._map_account_type(local_acc.type),
        }
        created = self.accounts_api.store_account(new_acc)
        firefly_id = int(created.data.id)
        self.account_map[local_acc.id] = firefly_id
        return firefly_id

    def export_ledger(self, service: AccountingService, ledger: Ledger):
        for acc in ledger.accounts:
            self._create_or_get_account(acc)
        
        for tx in ledger.transactions:
            # Assume simple two-way transaction: one source (credit), one dest (debit)
            source_id = None
            dest_id = None
            total_amount = Decimal('0')
            for entry in tx.entries:
                acc = next(a for a in ledger.accounts if a.id == entry.account_id)
                acc_id = self._create_or_get_account(acc)
                total_amount += entry.amount
                if entry.is_debit:
                    dest_id = acc_id
                else:
                    source_id = acc_id
            
            if source_id and dest_id:
                # Withdrawal from source
                withdrawal = {
                    'type': 'withdrawal',
                    'date': tx.date.isoformat(),
                    'amount': str(total_amount),
                    'description': tx.description,
                    'source_id': source_id,
                    'destination_id': dest_id,
                }
                # Deposit to dest
                deposit = {
                    'type': 'deposit',
                    'date': tx.date.isoformat(),
                    'amount': str(total_amount),
                    'description': tx.description,
                    'source_id': source_id,
                    'destination_id': dest_id,
                }
                transaction_data = {
                    'transactions': [withdrawal, deposit]
                }
                self.transactions_api.store_transaction(transaction_data)