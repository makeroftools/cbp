<DOCUMENT filename="index.md">
# Accounting Project
A simple, robust Python accounting system with CLI, persistence, and Firefly III integration.# Accounting Project

A Python-based accounting system for managing ledgers, accounts, transactions, and integrations.

## Features

- **Double-entry bookkeeping** with validation.
- **CLI interface** for ledger management.
- **Persistence** in Parquet/Feather formats.
- **Firefly III integration** for export.
- **Configurable** precision, rounding, logging.
- **Pydantic models** for type safety.
- **Unit tests** with 95%+ coverage.

## Installation

1. Install via Pixi: `pixi install`.
2. Run CLI: `pixi run app --help`.

## Quick Start

```bash
# Create ledger
pixi run app create "My Ledger" --context personal

# Add accounts
pixi run app add-account "Cash" --type asset
pixi run app add-account "Salary" --type revenue

# Add transaction
pixi run app add-transaction "Salary payment" --date 2025-11-12

# View trial balance
pixi run app trial-balance

# Save
pixi run app save ./ledger

Architecture
- Models: Pydantic-based entities (Account, Transaction, Ledger).
- Service: Core logic for additions, balances.
- Persistence: Polars DataFrames for storage.
- App: Typer CLI with Rich UI.
- Integrations: Firefly III API client.
See Usage for details.


