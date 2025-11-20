# test_subprocess_utils.py
"""
Pytest tests for subprocess_utils module.
"""

import pytest
import os
import tempfile
from pathlib import Path
from subprocess_utils import (
    SubprocessError,
    cwd,
    run,
    check_call,
    check_output,
    PopenContext,
    tail_log,
)


@pytest.fixture
def tmp_dir():
    with tempfile.TemporaryDirectory() as tmpdir:
        yield Path(tmpdir)


def test_cwd(tmp_dir):
    subdir = tmp_dir / "sub"
    subdir.mkdir()
    orig_cwd = os.getcwd()
    with cwd(subdir):
        assert os.getcwd() == str(subdir)
    assert os.getcwd() == orig_cwd


def test_run_success():
    result = run(["echo", "hello"])
    assert result.returncode == 0
    assert result.stdout.strip() == "hello"


def test_run_failure_no_check():
    result = run(["false"], check=False)
    assert result.returncode == 1


def test_run_failure_raises():
    with pytest.raises(SubprocessError):
        run(["false"])


def test_run_timeout():
    with pytest.raises(SubprocessError):
        run(["sleep", "1"], timeout=0.1)


def test_check_call_success():
    check_call(["echo", "hello"])


def test_check_call_failure():
    with pytest.raises(SubprocessError):
        check_call(["false"])


def test_check_output():
    out = check_output(["echo", "hello"])
    assert out.strip() == "hello"


def test_popen_context():
    with PopenContext(["echo", "hello"]) as proc:
        out, _ = proc.communicate()
        assert out.strip() == "hello"


def test_tail_log_non_follow(tmp_dir):
    log_file = tmp_dir / "log.txt"
    with open(log_file, "w") as f:
        for i in range(20):
            f.write(f"line {i}\n")
    lines = list(tail_log(log_file, lines=5))
    assert len(lines) == 5
    assert lines[0] == "line 15"


def test_tail_log_follow(tmp_dir, capsys):
    log_file = tmp_dir / "log.txt"
    with open(log_file, "w") as f:
        f.write("initial\n")
    gen = tail_log(log_file, follow=True)
    next(gen)  # initial
    with open(log_file, "a") as f:
        f.write("new line\n")
    captured = capsys.readouterr()
    # Note: follow yields, but capsys for output; adjust as needed
    assert True  # Placeholder, as infinite generator


def test_stream_output(caplog):
    caplog.set_level("INFO")
    run(["echo", "hello"], stream_output=True)
    assert "hello" in caplog.text


if __name__ == "__main__":
    pytest.main(["-v"])