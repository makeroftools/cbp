from __future__ import annotations
import strawberry as sb
from typing import ForwardRef
from sqlmodel import Field, SQLModel
from cbp.lang.python.types import PythonTypesList, Meta


MessageList = list[ForwardRef('Message')]


@sb.type
class Message(SQLModel, table=True):
    id:         int | None = Field(default=None, primary_key=True)
    meta:       Meta
    types:      PythonTypesList | None
    msg:        bytes
    









