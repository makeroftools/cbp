from decimal import Decimal, getcontext
import decimal
import logging
from accounting.models import Account, Transaction, Ledger, AccountType, Context, TransactionEntry
from accounting.config import AccountingConfig
from accounting.persistence import save_ledger, load_ledger

class AccountingService:
    def __init__(self, config: AccountingConfig):
        self.config = config
        getcontext().prec = self.config.precision
        getcontext().rounding = getattr(decimal, self.config.rounding)
        logging.basicConfig(level=getattr(logging, self.config.log_level))
        self.logger = logging.getLogger(__name__)

    def add_account(self, ledger: Ledger, account: Account) -> None:
        if self.config.validate_on_add and any(a.id == account.id for a in ledger.accounts):
            raise ValueError('Account ID already exists')
        ledger.accounts.append(account)
        self.logger.info(f"Added account: {account.name}")

    def add_transaction(self, ledger: Ledger, transaction: Transaction) -> None:
        if self.config.validate_on_add and transaction.context != ledger.context:
            raise ValueError('Transaction context mismatch')
        ledger.transactions.append(transaction)
        for entry in transaction.entries:
            account_found = False
            for acc in ledger.accounts:
                if acc.id == entry.account_id:
                    if entry.is_debit:
                        acc.balance += entry.amount
                    else:
                        acc.balance -= entry.amount
                    account_found = True
                    break
            if not account_found:
                raise ValueError('Account not found for entry')
        if self.config.log_transactions:
            self.logger.info(f"Added transaction: {transaction.description}")

    def get_account_balance(self, ledger: Ledger, account_id: str) -> Decimal:
        for acc in ledger.accounts:
            if acc.id == account_id:
                return acc.balance
        raise ValueError('Account not found')

    def generate_trial_balance(self, ledger: Ledger) -> dict:
        balances = {}
        for acc in ledger.accounts:
            balances[acc.name] = str(acc.balance)
        return balances

    def save(self, ledger: Ledger, path: str) -> None:
        save_ledger(ledger, path, self.config.persistence_format)
        self.logger.info(f"Saved ledger to {path}")

    def load(self, path: str) -> Ledger:
        loaded = load_ledger(path, self.config.persistence_format)
        self.logger.info(f"Loaded ledger from {path}")
        return loaded