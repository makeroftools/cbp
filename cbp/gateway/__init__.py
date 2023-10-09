import typer 


gw_app = typer.Typer()


@gw_app.command("deploy")
def deploy_gateway():
    """
    This is the gateway launcher
    """
    typer.echo("deploying gateway")


