from dataclasses import dataclass
import uuid 


@dataclass()
class PythonComponentConfig:
    name:           str
    uuid:           str = uuid.uuid4().hex[:4]
    pypi:           str
    exe:            str
    repo:           str 
    python_path:    str
    src_code_path:  str


def generate_python_component(python_component_config: PythonComponentConfig) -> None:
    """
    Goal: Create either a proc or coroutine 
    Recipe:
        1) determine if exe or package_call
        2) if package_call determine if in-expensive for async loop else launch multiproc
        3) if exe, launch multiproc
    Ingredients:
        * see the config
    Notes:
        * exes are easy, they have to be procs
        * coroutines need to be:
            * preferably available via package API
    """