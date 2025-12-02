import typer
import subprocess

app = typer.Typer()

@app.command()
def host_info(name: str):
    typer.echo(subprocess.check_output(["lspcu"]).decode())

@app.command()
def goodbye(name: str):
    typer.echo(f"Goodbye {name}!")
