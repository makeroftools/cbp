from __future__ import annotations
import strawberry as sb
from typing import Any, Optional
from sqlmodel import Field, SQLModel



@sb.type
class Message(SQLModel, table=True):
    msg_id:     Optional[int] = Field(default=None, primary_key=True)
    datetime:   str
    msg:        Any