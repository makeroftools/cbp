from __future__ import annotations
from typing import ForwardRef, Optional
from sqlmodel import Field, SQLModel
import strawberry as sb
import zmq
from cbp.lang.python.types import Meta

CbpSocketList = list[ForwardRef('CbpSocket')]


@sb.type
class CbpSocket(SQLModel, table=True):
    """
    Hey this is CBP Socket, how about that.
    """
    id:         int | None = Field(default=None, primary_key=True)
    meta:       Meta
    s_type:     zmq.SocketType
    bind:       bool                # if False, then connect
    address:    str
    subscribe:  str
    socket:     zmq.Socket | None



