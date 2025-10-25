import strawberry

from fastapi import FastAPI, Request
from fastapi.templating import Jinja2Templates
from fastapi.responses import HTMLResponse
from strawberry.asgi import GraphQL

# from cbp_python.schemas.movies import schema
from cbp_python.schemas.books import schema

app = FastAPI()

templates = Jinja2Templates(directory="src/cbp_python/templates")

@app.get("/", response_class=HTMLResponse)
async def render_index_page(request:Request):
    return templates.TemplateResponse("index.html", {"request": request})



app.mount("/graphql", GraphQL(schema=schema))