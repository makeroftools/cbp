import {{cookiecutter.project_slug}}._{{ cookiecutter.component_type}} as ct #

from dataclasses import dataclass


import typer

app = typer.Typer()

# A completely generic component must be distinguished/created as either a lang server, compute cpu, co-routine



app.add_typer(ct.app)