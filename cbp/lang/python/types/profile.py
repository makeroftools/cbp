from __future__ import annotations
import strawberry as sb
from typing import Optional
from sqlmodel import Field, SQLModel


@sb.type
class Profile(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)
    duration:   float
    cpu:        float
    mem:        float

