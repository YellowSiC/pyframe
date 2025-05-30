import inspect
from typing import Any, Callable, Dict, Optional, ParamSpec, TypeVar
from uuid import UUID, uuid4

from ..executers.executer import ConnectionsProtocol, ProtocolHandlerBase

P = ParamSpec("P")
R = TypeVar("R")


class MenuAPI(ProtocolHandlerBase):
    def __init__(self):
        self._callbacks: Dict[UUID, Dict[str, Any]] = {}
        ConnectionsProtocol.add_protocol("menu", self)

    def register(self, callback: Callable[P, R], *args: P.args, **kwargs: P.kwargs) -> UUID:
        event_id = uuid4()
        self._callbacks[event_id] = {
            "callback": callback,
            "args": args,
            "kwargs": kwargs,
        }
        return event_id

    async def trigger(self, data: Optional[Dict[str, Any]]) -> Any:
        data = data or {}

        payload: dict = data.get("payload", data)

        command_id_str = payload.get("command_id")
        if not command_id_str:
            return

        try:
            command_id = UUID(command_id_str)
        except ValueError:
            return

        entry = self._callbacks.get(command_id)
        if not entry:
            return

        cb: Callable[..., Any] = entry["callback"]
        args = entry["args"] + tuple(payload.get("extra_args", []))
        kwargs = {**entry["kwargs"], **payload.get("extra_kwargs", {})}
        sig = inspect.signature(cb)
        param_count = len(sig.parameters)
        # print(f"Signatur von {cb}: {sig} mit {param_count} Parametern.")
        if param_count == 0:
            if inspect.iscoroutinefunction(cb):
                await cb()
            else:
                cb()
        else:
            if inspect.iscoroutinefunction(cb):
                await cb(*args, **kwargs)
            else:
                cb(*args, **kwargs)

system_registry = MenuAPI()
