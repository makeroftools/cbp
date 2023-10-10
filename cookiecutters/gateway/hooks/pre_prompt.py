import sys 
import subprocess


cmds = [
    ["poetry", "add", "pyzmq"],
    ["poetry", "add", "fastapi[all]"],
    ["poetry", "add", "strawberry-graphql[fastapi]"]
]


def run_cmd(cmd) -> bool:
    try:
        subprocess.run(cmd, capture_output=True, check=True)
        print(f"Pre-prompt COMPLETED: {cmd}")
        return True
    except Exception:
        return False

def run():
    for cmd in cmds:
        run_cmd(cmd)


if __name__ == "__main__":
    run()