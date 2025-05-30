import asyncio
import uuid
from typing import Any, Dict, Optional

from _pyframe import create_webview

from .. import outbox


def run_webview(config_json: str) -> None:
    """
    Launches the webview using the provided JSON configuration.

    Args:
        config_json (str): JSON string containing the configuration for the webview.
    """
    try:

        create_webview(config_json)
    except Exception as e:
        print("[Webview-Fehler]:", e)


# Global queues for managing state updates and method invocations
state_scope: asyncio.Queue[Dict[str, Any]] = asyncio.Queue()
methods_scope: asyncio.Queue[Dict[str, Any]] = asyncio.Queue()

# Mapping of request IDs to pending asyncio Futures
_pending_responses: Dict[str, asyncio.Future[Any]] = {}


async def handle_window_response(response: Dict[str, Any]) -> None:
    """
    Handles responses from the webview by resolving the corresponding future.

    Args:
        response (Dict[str, Any]): The response data, including a unique request ID and result or error.

    Raises:
        Exception: If the response includes an error.
    """
    req_id: Optional[str] = response.get("id")
    if req_id and req_id in _pending_responses:
        future: asyncio.Future[Any] = _pending_responses.pop(req_id)

        if "error" in response:
            future.set_exception(Exception(response.get("error", "Unknown error")))
            return

        # If there is no "result" key, resolve with None
        future.set_result(response.get("result"))


async def endless_state_loop() -> None:
    """
    Continuously processes tasks from the state and methods queues.

    Emits events to the webview and handles potential errors.
    """
    while True:
        task: Optional[Dict[str, Any]] = None
        queue: Optional[asyncio.Queue[Dict[str, Any]]] = None

        # Prioritize state_scope over methods_scope
        if not state_scope.empty():
            queue = state_scope
        elif not methods_scope.empty():
            queue = methods_scope

        if queue:
            task = await queue.get()

        if task is None:
            await asyncio.sleep(0.01)
            continue

        future: Optional[asyncio.Future[Any]] = task.pop("future", None)
        data: Dict[str, Any] = task.get("data", {})

        try:
            outbox.emit_event("window_request", data)
        except Exception as e:
            if future:
                future.set_exception(e)


async def request(method: str, args: Dict[str, Any], scope: bool = True) -> Any:
    """
    Sends a request to the webview and waits for the corresponding response.

    Args:
        method (str): The name of the method to invoke.
        args (Dict[str, Any]): Arguments for the method.
        scope (bool, optional): Determines which queue to use. Defaults to True (state_scope).

    Returns:
        Any: The result from the webview.

    Raises:
        Exception: If the webview responds with an error.
    """
    req_id: str = str(uuid.uuid4())
    future: asyncio.Future[Any] = asyncio.get_event_loop().create_future()
    _pending_responses[req_id] = future

    queue = state_scope if scope else methods_scope
    await queue.put(
        {"data": {"id": req_id, "method": method, "args": args}, "future": future}
    )

    # Optional: You could wrap the future with asyncio.wait_for(...) for a timeout
    return await future
