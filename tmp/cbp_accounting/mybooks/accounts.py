from decimal import Decimal
from pydoc import describe
from ..models.models import Account, AccountType, Context, Transaction, TransactionEntry



accounts = [
    Account(
        name = "Wells Fargo Visa",
        type = AccountType.LIABILITY,
        context = Context.PERSONAL,
        balance = Decimal("0.00"),
    ),
    Account(
        name = "First Bank Checking",
        type = AccountType.ASSET,
        context = Context.PERSONAL,
        balance = Decimal("0.00"),
    ),
    Account(
        name = "Lakeland Visa",
        type = AccountType.LIABILITY,
        context = Context.PERSONAL,
        balance = Decimal("0.00")
    ),
    Account(
        name = "First Bank Visa",
        type = AccountType.LIABILITY,
        context = Context.PERSONAL,
        balance = Decimal("0.00")
    ),
    Account(
        name = "Fifth Third 6864",
        type = AccountType.ASSET,
        context = Context.BUSINESS,
        balance = Decimal("0.00")
    ),
    Account(
        name = "Citi Bank Visa",
        type = AccountType.LIABILITY,
        context = Context.PERSONAL,
        balance = Decimal("0.00")
    ),
    Account(
        name = ""
    )
]