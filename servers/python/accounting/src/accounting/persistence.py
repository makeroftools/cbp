import polars as pl
import json
import os
from decimal import Decimal
from datetime import datetime
from accounting.models import Account, Transaction, TransactionEntry, Context, AccountType, Ledger

def save_ledger(ledger: Ledger, path: str, format: str = 'parquet'):
    os.makedirs(path, exist_ok=True)
    accounts_data = [
        {
            'id': a.id,
            'name': a.name,
            'type': a.type.value,
            'context': a.context.value,
            'balance': str(a.balance),
            'description': a.description
        }
        for a in ledger.accounts
    ]
    df_accounts = pl.DataFrame(accounts_data)
    if format == 'parquet':
        df_accounts.write_parquet(f'{path}/accounts.parquet')
    else:
        df_accounts.write_ipc(f'{path}/accounts.feather')

    transactions_data = [
        {
            'id': t.id,
            'date': t.date.isoformat(),
            'description': t.description,
            'context': t.context.value
        }
        for t in ledger.transactions
    ]
    df_transactions = pl.DataFrame(transactions_data)
    if format == 'parquet':
        df_transactions.write_parquet(f'{path}/transactions.parquet')
    else:
        df_transactions.write_ipc(f'{path}/transactions.feather')

    entries_data = []
    for t in ledger.transactions:
        for e in t.entries:
            entries_data.append({
                'transaction_id': t.id,
                'account_id': e.account_id,
                'amount': str(e.amount),
                'is_debit': e.is_debit
            })
    df_entries = pl.DataFrame(entries_data)
    if format == 'parquet':
        df_entries.write_parquet(f'{path}/entries.parquet')
    else:
        df_entries.write_ipc(f'{path}/entries.feather')

    metadata = {
        'id': ledger.id,
        'name': ledger.name,
        'context': ledger.context.value
    }
    with open(f'{path}/metadata.json', 'w') as f:
        json.dump(metadata, f)

def load_ledger(path: str, format: str = 'parquet') -> Ledger:
    with open(f'{path}/metadata.json', 'r') as f:
        metadata = json.load(f)
    ledger = Ledger(
        id=metadata['id'],
        name=metadata['name'],
        context=Context(metadata['context'])
    )

    if format == 'parquet':
        df_accounts = pl.read_parquet(f'{path}/accounts.parquet')
    else:
        df_accounts = pl.read_ipc(f'{path}/accounts.feather')
    for row in df_accounts.iter_rows(named=True):
        account = Account(
            id=row['id'],
            name=row['name'],
            type=AccountType(row['type']),
            context=Context(row['context']),
            balance=Decimal('0'),  # Reset, recompute later
            description=row.get('description')
        )
        ledger.accounts.append(account)

    if format == 'parquet':
        df_transactions = pl.read_parquet(f'{path}/transactions.parquet')
        df_entries = pl.read_parquet(f'{path}/entries.parquet')
    else:
        df_transactions = pl.read_ipc(f'{path}/transactions.feather')
        df_entries = pl.read_ipc(f'{path}/entries.feather')

    trans_dict = {row['id']: row for row in df_transactions.iter_rows(named=True)}
    entries_dict = {}
    for row in df_entries.iter_rows(named=True):
        tid = row['transaction_id']
        if tid not in entries_dict:
            entries_dict[tid] = []
        entries_dict[tid].append(row)

    for trans_id, trans_row in trans_dict.items():
        entries_rows = entries_dict.get(trans_id, [])
        entries = [
            TransactionEntry(
                account_id=r['account_id'],
                amount=Decimal(r['amount']),
                is_debit=r['is_debit']
            )
            for r in entries_rows
        ]
        transaction = Transaction(
            id=trans_id,
            date=datetime.fromisoformat(trans_row['date']),
            description=trans_row['description'],
            entries=entries,
            context=Context(trans_row['context'])
        )
        ledger.transactions.append(transaction)

    # Recompute balances from transactions
    for transaction in ledger.transactions:
        for entry in transaction.entries:
            for acc in ledger.accounts:
                if acc.id == entry.account_id:
                    if entry.is_debit:
                        acc.balance += entry.amount
                    else:
                        acc.balance -= entry.amount
                    break

    return ledger