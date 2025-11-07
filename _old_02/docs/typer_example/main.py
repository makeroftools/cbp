import typer
import users 


app = typer.Typer(no_args_is_help=True)

app.add_typer(users.app, name="users")

if __name__ == "__main__":
    app()