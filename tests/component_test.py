from cbp.lang.python.component_engine import Component

ACK = "ok, task completed"

def task1(msg: bytes):
    return "ACK"

def task2(msg: bytes):
    print(msg)


class TestComponent:

