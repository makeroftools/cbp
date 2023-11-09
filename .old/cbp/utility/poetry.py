"""
subprocess calls to poetry
"""
from cbp.utility.procs import run_subprocess



async def update_poetry():
    await run_subprocess('poetry update')