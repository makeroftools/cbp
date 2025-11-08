import typer 

app = typer.Typer(name="query")


@app.command()
def blah():
    ...









if __name__ == '__main__':
    app()