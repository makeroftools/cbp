import strawberry

from fastapi import FastAPI, Depends, Request, WebSocket, BackgroundTasks
from strawberry.types import Info
from strawberry.fastapi import BaseContext, GraphQLRouter
from sqlmodel import create_engine
import zmq
import asyncio
from zmq.asyncio import Context, Poller


class Context(BaseContext):
    def __init__(self):
        """
        Used for zmq componentry
        Used for sqlite DB interface
        """
        self.db         = create_engine("sqlite:///gateway.db")
        self.zmq_ctx    = Context.instance()
        self.zmq_socket = self.zmq_ctx.socket(zmq.ROUTER)



def custom_context_dependency() -> Context:
    return Context()




async def get_context(
    custom_context=Depends(custom_context_dependency),
):
    return custom_context




@strawberry.type
class Query:
    @strawberry.field
    def example(self, info: Info) -> str:
        return f"Hello {info.context.name}, {info.context.greeting}"




schema = strawberry.Schema(Query)


graphql_app = GraphQLRouter(
    schema,
    context_getter=get_context,
)


app = FastAPI()
app.include_router(graphql_app, prefix="/graphql")