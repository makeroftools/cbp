import typer
from cbp.utility.generators import generators_app

app = typer.Typer()


app.add_typer(generators_app, name='generate')





if __name__ == '__main__':
    app()