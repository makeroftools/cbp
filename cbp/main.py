import typer


app = typer.Typer()

@app.command()
def generate(what: str = "component") -> int:
    """
    Command used to generate an artifact needed to run in a cbp app
    """
    match what:
        case "component":
            



@app.command()
def goodbye():
    pass



if __name__ == "__main__":
    app()