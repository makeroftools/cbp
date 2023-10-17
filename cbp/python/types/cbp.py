from __future__ import annotations
import zmq
from dataclasses import dataclass
from typing import Any, Callable

from ..generators import generate_id




@dataclass
class Profile:
    duration:   float


@dataclass
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

    

@dataclass
class Socket:
    """
    Hey this is CBP_Socket, how about that.
    """
    s_type:     zmq.SocketType
    bind:       bool                # if False, then connect
    address:    str
    subscribe:  bytes
    socket:     zmq.Socket


@dataclass
class Message:
    msg_id:     int
    datetime:   str
    msg:        Any

