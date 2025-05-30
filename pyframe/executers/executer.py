import inspect
from typing import (Any, Awaitable, Callable, Dict, Optional, ParamSpec, Tuple,
                    Type, TypeVar, Union, overload)

from pydantic import BaseModel, ValidationError


class ConnectionsProtocol:
    _protocols: Dict[str, "ProtocolHandlerBase"] = {}

    @classmethod
    def add_protocol(cls, protocol: str, protocol_handler: "ProtocolHandlerBase") -> None:
        cls._protocols[protocol] = protocol_handler

    @classmethod
    async def get_protocol(cls, protocol: str, data: Dict[str, Any]) -> Dict[str, Any]:
        handler = cls._protocols.get(protocol)
        if handler is None:
            raise ValueError(f"Protocol '{protocol}' not found!")
        result = await handler.trigger(data)
        return {"protocol": protocol, "result": result}


class ProtocolHandlerBase:
    async def trigger(self, data: Optional[Dict[str, Any]]) -> Any:
        raise NotImplementedError





