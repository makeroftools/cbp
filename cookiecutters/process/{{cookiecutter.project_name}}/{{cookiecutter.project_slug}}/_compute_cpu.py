from dataclasses import dataclass
from typing import Any, Callable, Optional
import zmq
from zmq.asyncio import Context, Poller


# Still need to fix message dataclass and send/recv msgs



@dataclass
class CBP_Socket:
    """
    Hey this is CBP_Socket, how about that.
    """
    s_type:     zmq.SocketType
    bind:       bool                # if False, then connect
    address:    str
    subscribe:  bytes
    socket:     zmq.Socket

@dataclass
class CBP_Message:
    msg_id:     int
    datetime:   str
    msg:        Any


class {{cookiecutter.component_name}}:
    """
    """
    def __init__(self, input_sockets: list[CBP_Socket], output_sockets: list[CBP_Socket], task: Optional[Callable]) -> None:
        self.input_sockets = input_sockets
        self.output_sockets = output_sockets
        self.ctx = Context.instance()
        self.poller = Poller()
        self.task = task



    def _add_socket(self, is_input: bool, socket: CBP_Socket) -> None:
        s = self.ctx.socket(socket.s_type)
        if socket.bind:
            s.bind(socket.address)
        else:
            s.connect(socket.address)
        if socket.subscribe:
            s.subscribe(socket.subscribe)

        socket.socket = s 

        if is_input:
            self.input_sockets.append(socket)
            self.poller.register(s)
        else:
            self.output_sockets.append(socket)


    async def run(self):
        """
        """
        while True:
            events = self.poller.poll()
            for s in self.input_sockets:
                if s.socket in events:
                    msg = await s.recv_multipart()
                    ret = self.task(msg)
                    for o in self.output_sockets:
                        await o.send_multipart(ret)


    def kill(self):
        for is_ in self.input_sockets:
            is_.close()
        for os_ in self.output_sockets:
            os_.close()
