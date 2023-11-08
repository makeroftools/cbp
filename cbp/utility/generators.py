from dataclasses import dataclass
import uuid
import os, shutil, sys
from pathlib import Path
import typer

from cbp.languages.python.types import Task
from cbp.utility.procs import run_subprocess

generators_app = typer.Typer()

def generate_id():
    return uuid.uuid4().hex[:4]


@generators_app.command('python-task-server')
def generate_python_task_server(where: Path):
    """
    Creates a python task server using FastAPI and Strawberry.
    Establishes communication between it and the gateway.
    Recipe:
    ======
        1. Docker container environment
            * pyproject.toml
                * use `generate_pyproject_file()`
            * Dockerfile
                * image is built locally
            * docker-compose and docker-swarm
        2. LOCAL only atm
            
    """
    shutil.copytree('./templates/pytaskserver', './')
    run_subprocess('docker compose up -d')

        
@generators_app.command('cbp-gateway-server')
def generate_cbpgateway(where: Path):
    """
    This is the user's access to the "portal"
    For now its just a request router and html server
    From here, requests are dispersed to language specific task servers
    """


def generate_pyproject_file(
        project_name: str, 
        deps: list[str], 
        dev_deps: list[str],
        project_dir: str
    ) -> None:
    """
    For dynamically generating python projects with latest dependencies
    Recipe:
    ======
        1. create a tmp directory
        2. `poetry new projname` ..etc
        3. copy completed pyproject.toml to templated project directory
        4. delete tmp dir
    """
    os.mkdir('tmp')
    os.chdir('./tmp')
    run_subprocess(f'poetry new {project_name}')

    for dep in deps:
        run_subprocess(f'poetry add {dep}')
    for ddep in dev_deps:
        run_subprocess(f'poetry add -D {ddep}')

    shutil.copyfile('./pyproject.toml', project_dir)

    os.chdir('..')
    os.remove('./tmp')

