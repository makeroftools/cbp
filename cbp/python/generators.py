from dataclasses import dataclass
import uuid

from .types.cbp import Task


def generate_id():
    return uuid.uuid4().hex[:4]





def generate_task(task: Task):
    """
    Takes as input a description or 'recipe'
    """
    if task.poetry_cmd:

