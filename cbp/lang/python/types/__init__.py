from .message import Message, MessageList
from .profile import Profile
from .socket import Socket, SocketList
from .component import Component, ComponentDict, ComponentList
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




