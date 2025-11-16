# subprocess_utils.py
"""
Robust subprocess utilities module.

Provides enhanced subprocess handling with timeouts, streaming, error management,
and context manager support.
"""

import subprocess
import sys
import typing as t
from contextlib import contextmanager
from pathlib import Path
import logging
import os
import time

logger = logging.getLogger(__name__)


class SubprocessError(Exception):
    """Custom exception for subprocess failures."""


@contextmanager
def cwd(path: t.Union[str, Path]):
    """Context manager to temporarily change working directory."""
    prev_cwd = Path.cwd()
    try:
        os.chdir(path)
        yield
    finally:
        os.chdir(prev_cwd)


def run(
    cmd: t.Union[str, t.List[str]],
    timeout: t.Optional[float] = None,
    check: bool = True,
    capture_output: bool = True,
    shell: bool = False,
    env: t.Optional[t.Dict[str, str]] = None,
    cwd: t.Optional[t.Union[str, Path]] = None,
    stream_output: bool = False,
    log_level: int = logging.INFO,
) -> subprocess.CompletedProcess:
    """
    Run a command with enhanced options.

    Args:
        cmd: Command to run.
        timeout: Timeout in seconds.
        check: Raise on non-zero exit.
        capture_output: Capture stdout/stderr.
        shell: Run as shell command.
        env: Environment variables.
        cwd: Working directory.
        stream_output: Stream output to console/log.
        log_level: Log level for output.

    Returns:
        CompletedProcess instance.
    """
    if stream_output and capture_output:
        raise ValueError("Cannot stream and capture output simultaneously.")

    kwargs = {
        "timeout": timeout,
        "check": check,
        "shell": shell,
        "env": env or {**os.environ},
        "capture_output": capture_output,
        "text": True,
    }
    if cwd:
        kwargs["cwd"] = str(cwd)

    try:
        proc = subprocess.run(cmd, **kwargs)
        if stream_output:
            if proc.stdout:
                logger.log(log_level, proc.stdout)
            if proc.stderr:
                logger.log(log_level - 10, proc.stderr)  # Log stderr as warning
        return proc
    except subprocess.TimeoutExpired as e:
        raise SubprocessError(f"Command timed out: {cmd}") from e
    except subprocess.CalledProcessError as e:
        if not check:
            return e
        raise SubprocessError(f"Command failed with code {e.returncode}: {cmd}") from e


def check_call(
    cmd: t.Union[str, t.List[str]],
    timeout: t.Optional[float] = None,
    **kwargs,
) -> None:
    """Run command and raise if non-zero exit."""
    run(cmd, timeout=timeout, check=True, **kwargs)


def check_output(
    cmd: t.Union[str, t.List[str]],
    timeout: t.Optional[float] = None,
    **kwargs,
) -> str:
    """Run command and return stdout if success."""
    proc = run(cmd, timeout=timeout, check=True, capture_output=True, **kwargs)
    return proc.stdout


class PopenContext:
    """Context manager for Popen with auto-cleanup."""

    def __init__(
        self,
        cmd: t.Union[str, t.List[str]],
        timeout: t.Optional[float] = None,
        shell: bool = False,
        env: t.Optional[t.Dict[str, str]] = None,
        cwd: t.Optional[t.Union[str, Path]] = None,
    ):
        self.cmd = cmd
        self.timeout = timeout
        self.shell = shell
        self.env = env
        self.cwd = cwd
        self.proc: t.Optional[subprocess.Popen] = None

    def __enter__(self) -> subprocess.Popen:
        kwargs = {
            "shell": self.shell,
            "env": self.env or {**os.environ},
            "cwd": str(self.cwd) if self.cwd else None,
            "text": True,
            "bufsize": 1,  # Line buffered
        }
        self.proc = subprocess.Popen(self.cmd, **kwargs)
        return self.proc

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.proc:
            if self.timeout:
                try:
                    self.proc.wait(timeout=self.timeout)
                except subprocess.TimeoutExpired:
                    self.proc.kill()
                    raise SubprocessError("Popen timed out")
            else:
                self.proc.wait()
            self.proc = None


def tail_log(
    log_file: t.Union[str, Path],
    lines: int = 10,
    follow: bool = False,
) -> t.Generator[str, None, None]:
    """Yield last N lines from log file, optionally follow."""
    path = str(log_file)
    if not os.path.exists(path):
        return
    with open(path, "r") as f:
        if follow:
            f.seek(0, 2)
            while True:
                line = f.readline()
                if not line:
                    time.sleep(0.1)
                    continue
                yield line.rstrip()
        else:
            # Read all lines and take last N
            lines_list = f.readlines()
            for line in lines_list[-lines:]:
                yield line.rstrip()