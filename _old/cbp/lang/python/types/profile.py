from __future__ import annotations
import strawberry as sb
from sqlmodel import Field, SQLModel
from cbp.lang.python.types.meta import Meta


@sb.type
class Profile(SQLModel, table=True):
    id:         int | None = Field(default=None, primary_key=True)
    meta:       Meta
    duration:   float
    cpu:        float
    mem:        float

