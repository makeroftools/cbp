import asyncio 
import zmq 
from dataclasses import dataclass
from typing import Any, Optional
from collections.abc import Callable


@dataclass()
class SocketConfig:
    socket_type: int

@dataclass()
class ComponentConfig:
    socket_config: SocketConfig


class Component:
    def __init__(self, config: ComponentConfig, task: Callable[[Any],Any]) -> None:
        self.input_sockets = []
        self.output_sockets = []
        self.system_sockets = []
        self.ctx = zmq.

    def run(self):
        pass 

    def kill(self):


