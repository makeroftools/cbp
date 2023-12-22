import zmq
import asyncio
from zmq.asyncio import Context, Poller
import time

from dataclasses import dataclass

ctx = zmq.asyncio.Context()

url = 'tcp://127.0.0.1:5555'

async def ping() -> None:
    """ print dots to indicate idleness """
    while True:
        await asyncio.sleep(0.5)
        print('.')

async def receiver() -> None:
    """ receive message with polling """
    pull = ctx.socket(zmq.PULL)
    pull.connect(url)
    poller = Poller()
    poller.register(pull, zmq.POLLIN)
    while True:
        events = await poller.poll()
        if pull in dict(events):
            print('recving', events)
            msg = await pull.recv_multipart()
            print('recvd', msg)

async def sender() -> None:
    """ send a msg every second """
    tic = time.time()
    push = ctx.socket(zmq.PUSH)
    push.bind(url)
    push.bind(url)
    while True:
        print('sending')
        await push.send_multipart([str(time.time() - tic).encode('ascii')])
        await asyncio.sleep(1)

class Component:
    def __init__(self):
        pass

    def run(self):
        pass

    def kill(self):
        pass



