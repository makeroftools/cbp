import typer 
from cbp.tree_sitter.query import app as query_app
from cbp.tree_sitter.parse import app as parse_app

app = typer.Typer(name="tree-sitter", no_args_is_help=True)
app.add_typer(query_app)
app.add_typer(parse_app)



def main():
    ...














if __name__ == '__main__':
    main()
    