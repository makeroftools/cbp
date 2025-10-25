import strawberry as sb
from cbp_python.types.book import Book, get_books

@sb.type
class Query:
    books: list[Book] = sb.field(resolver=get_books)

schema = sb.Schema(Query)