import typer
from .gateway import gw_app

app = typer.Typer()



app.add_typer(gw_app, name="gateway")




