# app.py
from decimal import Decimal
from datetime import datetime
from typing import Optional
import typer
from rich.console import Console
from rich.table import Table
from rich.prompt import Prompt, Confirm

from accounting.models import Account, AccountType, Context, Transaction, TransactionEntry, Ledger
from accounting.config import AccountingConfig
from accounting.service import AccountingService
from accounting.integrations.firefly import FireflyIntegrator

app = typer.Typer()
console = Console()
config = AccountingConfig()
service = AccountingService(config)
current_ledger: Optional[Ledger] = None

@app.command()
def create(
    name: str = typer.Argument(..., help="Ledger name"),
    context: str = typer.Option("personal", "--context", help="Context (personal|business)")
):
    global current_ledger
    ctx = Context(context.upper())
    current_ledger = Ledger(name=name, context=ctx)
    console.print(f"[green]Created ledger '{name}'[/green]")

@app.command()
def add_account(
    name: str = typer.Argument(..., help="Account name"),
    type_: str = typer.Option("asset", "--type", help="Type (asset|liability|equity|revenue|expense)"),
    description: Optional[str] = typer.Option(None, "--description", help="Description")
):
    if not current_ledger:
        raise typer.Exit("No ledger. Run 'create' or 'load' first.", code=1)
    acc_type = AccountType(type_.upper())
    account = Account(name=name, type=acc_type, context=current_ledger.context, description=description)
    service.add_account(current_ledger, account)
    console.print(f"[green]Added account '{name}'[/green]")

@app.command()
def add_transaction(
    description: str = typer.Argument(..., help="Description"),
    date: Optional[str] = typer.Option(None, "--date", help="Date (YYYY-MM-DD)")
):
    if not current_ledger:
        raise typer.Exit("No ledger. Run 'create' or 'load' first.", code=1)
    entries = []
    while Confirm.ask("Add entry?"):
        account_id = Prompt.ask("Account ID")
        amount_str = Prompt.ask("Amount")
        amount = Decimal(amount_str)
        is_debit = Confirm.ask("Debit?")
        entry = TransactionEntry(account_id=account_id, amount=amount, is_debit=is_debit)
        entries.append(entry)
    dt = datetime.now() if date is None else datetime.fromisoformat(date)
    tx = Transaction(description=description, date=dt, entries=entries, context=current_ledger.context)
    service.add_transaction(current_ledger, tx)
    console.print(f"[green]Added transaction '{description}'[/green]")

@app.command()
def balance(account_id: str = typer.Argument(..., help="Account ID")):
    if not current_ledger:
        raise typer.Exit("No ledger. Run 'create' or 'load' first.", code=1)
    try:
        bal = service.get_account_balance(current_ledger, account_id)
        console.print(f"[blue]Balance for {account_id}: {bal}[/blue]")
    except ValueError:
        console.print("[red]Account not found[/red]")
        raise typer.Exit(1)

@app.command()
def trial_balance():
    if not current_ledger:
        raise typer.Exit("No ledger. Run 'create' or 'load' first.", code=1)
    trial = service.generate_trial_balance(current_ledger)
    table = Table(title="Trial Balance")
    table.add_column("Account", style="cyan")
    table.add_column("Balance", justify="right")
    for name, bal_str in trial.items():
        table.add_row(name, bal_str)
    console.print(table)

@app.command()
def save(path: str = typer.Argument(..., help="Save path")):
    if not current_ledger:
        raise typer.Exit("No ledger. Run 'create' or 'load' first.", code=1)
    service.save(current_ledger, path)
    console.print(f"[green]Saved to {path}[/green]")

@app.command()
def load(path: str = typer.Argument(..., help="Load path")):
    global current_ledger
    current_ledger = service.load(path)
    console.print(f"[green]Loaded from {path}[/green]")

@app.command()
def export_firefly(base_url: str = typer.Argument(..., help="Firefly III base URL"), token: str = typer.Argument(..., help="Access token")):
    if not current_ledger:
        raise typer.Exit("No ledger. Run 'create' or 'load' first.", code=1)
    integrator = FireflyIntegrator(config, base_url, token)
    integrator.export_ledger(service, current_ledger)
    console.print("[green]Exported to Firefly III[/green]")

if __name__ == "__main__":
    app()