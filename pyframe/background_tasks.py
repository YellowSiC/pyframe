from __future__ import annotations

import asyncio
import weakref
from typing import Any, Awaitable, Callable, Coroutine, Dict, Set, TypeVar

from . import core, helpers

running_tasks: Set[asyncio.Task] = set()
lazy_tasks_running: Dict[str, asyncio.Task] = {}
lazy_coroutines_waiting: Dict[str, Coroutine[Any, Any, Any]] = {}
functions_awaited_on_shutdown: weakref.WeakSet[Callable] = weakref.WeakSet()


def create(coroutine: Awaitable, *, name: str = "unnamed task") -> asyncio.Task:
    assert core.loop is not None
    coroutine = (
        coroutine
        if asyncio.iscoroutine(coroutine)
        else helpers.wait_for(coroutine, None)
    )
    task: asyncio.Task = core.loop.create_task(coroutine, name=name)
    task.add_done_callback(_handle_task_result)
    running_tasks.add(task)
    task.add_done_callback(running_tasks.discard)
    return task


def create_lazy(coroutine: Awaitable, *, name: str) -> None:
    if name in lazy_tasks_running:
        if name in lazy_coroutines_waiting:
            lazy_coroutines_waiting[name].close()
        lazy_coroutines_waiting[name] = _ensure_coroutine(coroutine)
        return

    def finalize(name: str) -> None:
        lazy_tasks_running.pop(name)
        if name in lazy_coroutines_waiting:
            create_lazy(lazy_coroutines_waiting.pop(name), name=name)

    task = create(coroutine, name=name)
    lazy_tasks_running[name] = task
    task.add_done_callback(lambda _: finalize(name))


F = TypeVar("F", bound=Callable)


def await_on_shutdown(func: F) -> F:
    functions_awaited_on_shutdown.add(func)
    return func


def _ensure_coroutine(awaitable: Awaitable[Any]) -> Coroutine[Any, Any, Any]:
    if asyncio.iscoroutine(awaitable):
        return awaitable

    async def wrapper() -> Any:
        return await awaitable

    return wrapper()


def _handle_task_result(task: asyncio.Task) -> None:
    try:
        task.result()
    except asyncio.CancelledError:
        pass
    except Exception as e:
        core.handle_exception(e)


async def teardown() -> None:
    while running_tasks or lazy_tasks_running:
        tasks = running_tasks | set(lazy_tasks_running.values())
        for task in tasks:
            if (
                not task.done()
                and not task.cancelled()
                and not _should_await_on_shutdown(task)
            ):
                task.cancel()
        if tasks:
            await asyncio.sleep(0)
            try:
                await helpers.wait_for(
                    asyncio.gather(*tasks, return_exceptions=True), timeout=2.0
                )
            except asyncio.TimeoutError:
                core.log.error(
                    "Could not cancel %s tasks within timeout: %s",
                    len(tasks),
                    ", ".join(t.get_name() for t in tasks if not t.done()),
                )
            except Exception:
                core.log.exception("Error while cancelling tasks")
    for coro in lazy_coroutines_waiting.values():
        coro.close()


def _should_await_on_shutdown(task: asyncio.Task) -> bool:
    try:
        return any(
            fn.__code__ is task.get_coro().cr_frame.f_code
            for fn in functions_awaited_on_shutdown
        )
    except AttributeError:
        return False
