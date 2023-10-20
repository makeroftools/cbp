from __future__ import annotations
import strawberry as sb
from typing import Any 



@sb.type
class Message:
    msg_id:     int
    datetime:   str
    msg:        Any