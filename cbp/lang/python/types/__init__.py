from .task import Task 
from .message import Message
from .profile import Profile
from .socket import Socket
from .component import Component
from .meta import Meta

from typing import ForwardRef
from enum import Enum


PythonTypesList = ForwardRef('PythonTypes')

class PythonTypes(Enum):
    str = 1
    bytes = 2
    int = 3
    float = 4
    datetime = 5
    object = 6




