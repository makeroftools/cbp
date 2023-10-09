from dataclasses import dataclass


@dataclass
class ComputeProfile:
    """
    execution time, algo complexity, 
    hardware (compute, can/should be virtual too, I guess) needed and settings
    * Co-routines act as a compute device (the routine (aka task) )
    """
    pass


@dataclass
class Task:
    name: str
    uid: str
    desc: str
    url_src: str 
    url_bin: str
    compute_profile: ComputeProfile



