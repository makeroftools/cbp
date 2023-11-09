import asyncio
import subprocess as sp
from dataclasses import dataclass



def run_subprocess(cmd: str | list[str]):
    """
    Run a shell command and obtain its result
    """
    
    if type(cmd) == type(str):
        cmd = cmd.split(' ')
    proc =  sp.run(cmd, stdout=sp.PIPE, text=True, check=True)
    
    return proc.stdout




async def run_subprocess_async(cmd: str | list[str]):
    """
    Run a shell command and obtain its result
    """
    if type(cmd) == type(list[str]):
        cmd = ' '.join(cmd)
        
    proc = await asyncio.create_subprocess_shell(
        cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE    )

    stdout, _ = await proc.communicate()

    return stdout




