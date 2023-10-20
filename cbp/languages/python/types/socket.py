from __future__ import annotations
from typing import Any 
import strawberry as sb
import zmq



@sb.type
class Socket:
    """
    Hey this is CBP_Socket, how about that.
    """
    s_type:     Any                  # zmq.SocketType
    bind:       bool                # if False, then connect
    address:    str
    subscribe:  bytes
    socket:     Any                    # zmq.Socket