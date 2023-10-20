import strawberry

from fastapi import FastAPI, Depends, Request, WebSocket, BackgroundTasks
from strawberry.types import Info
from strawberry.fastapi import BaseContext, GraphQLRouter




class Context(BaseContext):
    def __init__(self, greeting: str, name: str):
        """
        Used for zmq componentry
        Used for sqlite DB interface
        """




def custom_context_dependency() -> Context:
    return Context(greeting="you rock!", name="John")




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