from typing import Any, Callable, Optional
import zmq
import uuid
from zmq.asyncio import Context, Poller
from typing import ForwardRef
from dataclasses import dataclass

@dataclass
class State:
    input_sockets: list[zmq.Socket]
    output_sockets: list[zmq.Socket]


class Component:
    """
        This is it.. it is the miricle that provides.
        The magical all encompassing component.
        Its dynamic


    """

    ctx = Context.instance()
    poller = None                   # zmq Poller()



    def __init__(self, child: ForwardRef('Component'), use_poller: bool = False):
        self.child = child

    def add_sender(self):
        """
            Here we generate a socket to send output messages from.
            It will be unique to the zmq socket type assigned
                REQ, REP, PUSH, PUB, etc
            ..and the network topology
                connect VS bind
        """
        pass

    def add_receiver(self):
        """
            Generates a recver to the poller
            Creates the poller if None
        """