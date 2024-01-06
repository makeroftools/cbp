from typing import Any, Callable, Optional
import zmq
import uuid
from zmq.asyncio import Context, Poller
from zmq import REQ, REP, PUSH, PULL, PUB, SUB
from typing import ForwardRef
from cbp.lang.python.types.socket import CBPSocketList


zmq.Socket.send_multipart()

ZmqMultiMsg = list[bytes|zmq.Frame]


class Task:
    def __init__(self, callback: Callable):
        self.task = callable

    


class Component:
    """
        This is it.. it is the miricle that provides.
        The magical all encompassing component.
        Its dynamic
    """
    poller:         Poller | None                   = None 
    ctx:            Context                         = Context.instance()
    engine_running: bool                            = False
    components:     list[ForwardRef('Component')]   = []


    @classmethod()
    async def start_engine(cls):
        """
        """
        if cls.running:
            return
        while True:
            events = dict(await cls.poller.poll())
            for component in cls.components:
                for sock in component.input_sockets:
                    if sock in events:
                        msg = await sock.socket.recv_multipart()
                        ret = await component.on_recv(msg)
                        component.send(ret)



    def __init__(self, manifest: dict[str,Any], task: Task):
        self.input_sockets: CBPSocketList
        self.output_sockets: CBPSocketList
        self.task = task
        if self.poller == None:
            self.poller = Poller()

        for socket in manifest['sockets']:
            self.add_socket(socket)


    async def send(self, msg: str) -> None:
        """
        """
        for sock in self.output_sockets:
            await sock.send_multipart(msg)


    async def on_recv(self, msg: ZmqMultiMsg) -> ZmqMultiMsg:
        """
            NOTES: 
                * Should this be async or NO? 
        """
        msg = msg[-1]                       # FIXME: wire protocol frames
        return await self.task(msg)        



    # def add_socket(self, is_input: bool, bind: bool, stype: zmq.Socket, addr: str):
    def add_socket(self, socket: dict[str,Any]):

        """
            * Well, there is Socket type [REQ,REP,PUSH,PUB]
                * ..determined by type of task ?
            * there is.. bind or connect
            * finally register with poller and append to container
        """
        sock = self.ctx.socket(stype)
        if socket['bind']:
            sock.bind(socket['addr'])
        else:
            sock.connect(socket['addr'])
        
        if socket['is_input']:
            self.state.input_sockets.append(sock)
            self.poller.register(socket=sock, flags=zmq.POLLIN)
        else:
            self.state.output_sockets.append(sock)





    def kill(self):
        for sock in (self.input_sockets + self.output_sockets):
            sock.socket.close()

