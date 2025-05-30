import asyncio
import functools
import hashlib
import os
import socket
from collections.abc import Awaitable
from pathlib import Path
from typing import Any, Optional, Set, Union

import wait_for2

from . import core

# Track warnings to ensure they are only displayed once
_shown_warnings: Set[str] = set()


def warn_once(message: str, *, stack_info: bool = False) -> None:
    if message not in _shown_warnings:
        core.log.warning(message, stack_info=stack_info)
        _shown_warnings.add(message)


def is_pytest() -> bool:
    return "PYTEST_CURRENT_TEST" in os.environ


def is_coroutine_function(obj: Any) -> bool:
    while isinstance(obj, functools.partial):
        obj = obj.func
    return asyncio.iscoroutinefunction(obj)


def is_file(path: Optional[Union[str, Path]]) -> bool:
    if not path:
        return False
    if isinstance(path, str) and path.strip().startswith("data:"):
        return False  # Avoid data URLs
    try:
        return Path(path).is_file()
    except OSError:
        return False


def hash_file_path(path: Path) -> str:
    return hashlib.sha256(path.as_posix().encode()).hexdigest()[:32]


def is_port_open(host: str, port: int) -> bool:
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect((host, port))
    except (ConnectionRefusedError, TimeoutError, OSError):
        return False
    else:
        return True
    finally:
        sock.close()


async def wait_for(fut: Awaitable, timeout: Optional[float] = None) -> None:
    return await wait_for2.wait_for(fut, timeout)
