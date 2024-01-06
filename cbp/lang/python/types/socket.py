from __future__ import annotations
from typing import Any, Optional
from sqlmodel import Field, SQLModel
import strawberry as sb
import zmq



@sb.type
class CBPSocket(SQLModel, table=True):
    """
    Hey this is CBP Socket, how about that.
    """
    id:         Optional[int] = Field(default=None, primary_key=True)
    label:      Optional[str]
    s_type:     zmq.SocketType
    bind:       bool                # if False, then connect
    address:    str
    subscribe:  bytes
    socket:     zmq.Socket



CBPSocketList = list[CBPSocket]