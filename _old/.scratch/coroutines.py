import asyncio
import time

import zmq
from zmq.asyncio import Context, Poller

url = 'tcp://127.0.0.1:5555'

ctx = Context.instance()


async def ping() -> None:
    """print dots to indicate idleness"""
    while True:
        await asyncio.sleep(0.5)
        print('.')


async def receiver() -> None:
    """receive messages with polling"""
    pull = ctx.socket(zmq.PULL)
    pull.connect(url)
    poller = Poller()
    poller.register(pull, zmq.POLLIN)
    while True:
        try:
            events = await poller.poll()
        except Exception as e:
            print(f"Poller error: {e}")
            return

        if pull in dict(events):
            print("recving", events)
            msg = await pull.recv_multipart()
            print('recvd', msg)


async def sender() -> None:
    """send a message every second"""
    tic = time.time()
    push = ctx.socket(zmq.PUSH)
    push.bind(url)
    while True:
        print("sending")
        await push.send_multipart([str(time.time() - tic).encode('ascii')])
        await asyncio.sleep(1)


asyncio.run(
    asyncio.wait([
        asyncio.create_task(ping()),
        asyncio.create_task(receiver()),
        asyncio.create_task(sender()),
    ])
)
