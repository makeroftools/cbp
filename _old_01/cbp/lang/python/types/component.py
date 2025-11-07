from __future__ import annotations
import strawberry as sb
from typing import ForwardRef
from sqlmodel import Field, SQLModel

from cbp.lang.python.types import SocketList, Message, Meta, Recipe


ComponentList = list[ForwardRef('Component')]
ComponentDict = dict[int,list[ForwardRef('Component')]]

"""
    Remember, a component "knows how to build itself"
    ..abstractly, this is taken to mean a composite component is comprised of
    sticky-connections.. it's "flow-graph".
"""


@sb.type
class Component(SQLModel, table=True):
    id:             int | None = Field(default=None, primary_key=True)
    meta:           Meta
    input_sockets:  SocketList
    output_sockets: SocketList
    input_msgtype:  Message
    output_msgtype: Message
    source:         str                 # either a string of code or address to code


