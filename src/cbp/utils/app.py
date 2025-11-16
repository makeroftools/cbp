import typer 

app = typer.Typer()

@app.command()
def host_info(name: str):
    

@app.command()
def goodbye(name: str):
    typer.echo(f"Goodbye {name}!")

@app.command()
def goodbye(name: str):
    typer.echo(f"Goodbye {name}!")
