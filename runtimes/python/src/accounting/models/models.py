from decimal import Decimal
from pydantic import BaseModel, Field, field_validator
from typing import List, Optional
from uuid import uuid4
from datetime import datetime
from enum import Enum

class AccountType(Enum):
    ASSET = "asset"
    LIABILITY = "liability"
    EQUITY = "equity"
    REVENUE = "revenue"
    EXPENSE = "expense"

class Context(Enum):
    PERSONAL = "personal"
    BUSINESS = "business"

class Account(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid4()))
    name: str
    type: AccountType
    context: Context
    balance: Decimal = Decimal('0.00')
    description: Optional[str] = None

class TransactionEntry(BaseModel):
    account_id: str
    amount: Decimal
    is_debit: bool

    @field_validator('amount')
    @classmethod
    def amount_positive(cls, v):
        if v < 0:
            raise ValueError('Amount must be positive')
        return v

class Transaction(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid4()))
    date: datetime = Field(default_factory=datetime.now)
    description: str
    entries: List[TransactionEntry]
    context: Context

    @field_validator('entries')
    @classmethod
    def balanced_entries(cls, v):
        total_debit = sum(e.amount for e in v if e.is_debit)
        total_credit = sum(e.amount for e in v if not e.is_debit)
        if total_debit != total_credit:
            raise ValueError('Transaction must balance')
        return v

class Ledger(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid4()))
    name: str
    context: Context
    accounts: List[Account] = []
    transactions: List[Transaction] = []

