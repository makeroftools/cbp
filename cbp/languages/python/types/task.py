from __future__ import annotations
from typing import Any, Optional
from sqlmodel import Field, SQLModel
import strawberry as sb
from cbp.languages.python.types.profile import Profile
from cbp.languages.python.types.input import Input
from cbp.languages.python.types.output import Output


@sb.type
class Task(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)
    name: str
    description: str 
    composition: list[Task]
    deploymnent: str
    source: str 
    profile: Profile
    input: Input
    output: Output