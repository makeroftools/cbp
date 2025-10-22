import typer

app = typer.Typer(no_args_is_help=True)


@app.command()
def delete(user_name: str):
    print(f"Deleting user: {user_name}")


if __name__ == "__main__":
    app()