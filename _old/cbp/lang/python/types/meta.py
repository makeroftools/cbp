from __future__ import annotations
import strawberry as sb
from typing import ForwardRef
from sqlmodel import Field, SQLModel



@sb.type
class Meta(SQLModel, table=True):
    id:         int | None = Field(default=None, primary_key=True)
    timestamp:  str | None
    label:      str | None
    desc:       str | None
    

    
