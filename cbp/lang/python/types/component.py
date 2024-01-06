from __future__ import annotations
import strawberry as sb
from typing import
from sqlmodel import Field, SQLModel

from cbp.lang.python.types import CbpSocketList, Message, Meta


@sb.type
class Component(SQLModel, table=True):
    id:             int | None = Field(default=None, primary_key=True)
    meta:           Meta
    input_sockets:  CbpSocketList
    output_sockets: CbpSocketList
    input_msgtype:  Message
    output_msgtype: Message
    source:         str                 # either a string of code or address of code


