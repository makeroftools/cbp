from __future__ import annotations
from typing import Any
import strawberry as sb
from cbp.languages.python.types.profile import Profile

@sb.type
class Task:
    """
    A Task is: 
        * a composite of a particular task network that defines it
    """
    name:           str
    uid:            str
    description:    str 
    composition:    list[Task]
    poetry_cmd:     str
    src_str:        str 
    src_url:        str 
    repo_url:       str
    file_path:      str                                     # from project root to module that contains task
    profile:        Profile                                 # task exe duration (and other metrics)
    inputs:         tuple[list[Any], dict[Any, Any]]
