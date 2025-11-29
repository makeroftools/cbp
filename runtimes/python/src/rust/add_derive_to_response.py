#!/usr/bin/env python3
"""
Rust derive injector – Typer CLI

Adds specified traits to #[derive(...)] and/or injects additional attributes
before struct/enum definitions in openapi-generator Rust models.
"""

from pathlib import Path
import shutil
import jinja2
from typing import List, Optional
import typer

app = typer.Typer(
    name="derive-inject",
    help="Add derives and attributes to Rust structs from openapi-generator",
    add_completion=False,
)

# Template adds derives
DERIVE_TEMPLATE = """{% for line in lines %}
{% if line.strip().startswith("#[derive(") %}
{% set orig = line.strip()[9:] %}
{% set cleaned = orig.rstrip(")").strip() %}
#[derive({{ cleaned }}{% if cleaned %}, {% endif %}{{ added_derives }})
{% else %}
{{ line }}{% endif %}
{% endfor %}"""

# Template injects extra attributes before struct/enum
ATTR_TEMPLATE = """{% for i, line in enumerate(lines) %}
{% if i > 0 and lines[i-1].lstrip().startswith(("pub struct ", "pub enum ")) and not line.lstrip().startswith("#[") %}
{{ extra_attrs }}
{{ line }}{% else %}
{{ line }}{% endif %}
{% endfor %}"""

derive_env = jinja2.Environment(trim_blocks=True, lstrip_blocks=True)
attr_env = jinja2.Environment(trim_blocks=True, lstrip_blocks=True)

derive_tmpl = derive_env.from_string(DERIVE_TEMPLATE)
attr_tmpl = attr_env.from_string(ATTR_TEMPLATE)


def process_file(
    file_path: Path,
    added_derives: str,
    extra_attrs: str,
    dry_run: bool,
) -> None:
    content = file_path.read_text(encoding="utf-8")

    new_content = derive_tmpl.render(lines=content.splitlines(keepends=True), added_derives=added_derives)
    if extra_attrs:
        new_content = attr_tmpl.render(lines=new_content.splitlines(keepends=True), extra_attrs=extra_attrs)

    if content == new_content:
        typer.echo(f"No changes: {file_path}")
        return

    if dry_run:
        typer.echo(f"Would update: {file_path}")
        return

    backup = file_path.with_suffix(file_path.suffix + ".bak")
    shutil.copy2(file_path, backup)
    file_path.write_text(new_content, encoding="utf-8")
    typer.echo(f"Updated: {file_path} (backup: {backup.name})")


@app.command()
def main(
    path: Path = typer.Argument(..., help="Directory with Rust model files", exists=True, file_okay=False),
    derives: List[str] = typer.Option(
        ["Clone", "Default", "Debug", "PartialEq", "Serialize", "Deserialize"],
        "--derive",
        "-d",
        help="Traits to add to every #[derive(...)]",
    ),
    attrs: List[str] = typer.Option(
        [], "--attr", "-a", help="Additional attributes to insert before each struct/enum"
    ),
    pattern: str = typer.Option("*_response.rs", "--pattern", "-p", help="File glob pattern"),
    dry_run: bool = typer.Option(False, "--dry-run", "-n", help="Show changes without writing"),
):
    """Inject derives and attributes into Rust model files."""
    added_derives_str = ", ".join(derives)
    extra_attrs_str = "\n".join(attrs) + ("\n" if attrs else "")

    files = list(path.rglob(pattern))
    if not files:
        typer.echo("No files matched.")
        raise typer.Exit()

    typer.echo(f"Processing {len(files)} file(s)\n")
    for f in files:
        process_file(f, added_derives_str, extra_attrs_str, dry_run)

    if dry_run:
        typer.echo("\nDry-run complete.")
    else:
        typer.echo("\nDone.")


if __name__ == "__main__":
    app()