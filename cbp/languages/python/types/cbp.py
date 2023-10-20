from __future__ import annotations
import zmq
import strawberry as sb
from typing import Any, Callable

from cbp.utility.generators import generate_id




@sb.type
class Profile:
    duration:   float


@sb.type
class Task:
    """
    A Task is: 
        * a composite of a particular task network that defines it
    """
    name:           str
    uid:            str         = generate_id()
    description:    str 
    composition:    list[Task]
    poetry_cmd:     str

    

@sb.type
class Socket:
    """
    Hey this is CBP_Socket, how about that.
    """
    s_type:     zmq.SocketType
    bind:       bool                # if False, then connect
    address:    str
    subscribe:  bytes
    socket:     zmq.Socket


@sb.type
class Message:
    msg_id:     int
    datetime:   str
    msg:        Any

