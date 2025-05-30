import contextlib
import os
import sys
from pathlib import Path


@contextlib.contextmanager
def suppress_stderr():
    with open(os.devnull, "w") as fnull:
        saved = os.dup(sys.stderr.fileno())
        os.dup2(fnull.fileno(), sys.stderr.fileno())
        try:
            yield
        finally:
            os.dup2(saved, sys.stderr.fileno())
            os.close(saved)


def current_folder_name() -> str:
    return Path(Path.cwd()).resolve().parent.name
