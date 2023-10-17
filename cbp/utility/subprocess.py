import asyncio
from dataclasses import dataclass



    


async def run_subprocess(cmd):
    """
    Run a shell command and obtain its result
    """
    proc = await asyncio.create_subprocess_shell(
        cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE
    )

    stdout, _ = await proc.communicate()

    return stdout




