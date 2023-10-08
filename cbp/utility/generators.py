from dataclasses import dataclass
import uuid 


@dataclass()
class PythonComponentConfig:
    name: str
    uuid: str = uuid.uuid4().hex[:4]


def generate_python_component(python_component_config: PythonComponentConfig) -> None:
    