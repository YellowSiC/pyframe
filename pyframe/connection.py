# socketio_app.py
import inspect

# from .invoker import registry
from typing import Any, Awaitable, Callable, Dict, Optional, ParamSpec, TypeVar
from uuid import UUID, uuid4

import socketio

from . import core
from .executers.executer import ConnectionsProtocol
from .runtime import handle_window_response

P = ParamSpec("P")
R = TypeVar("R")


class SocketIoApp(socketio.ASGIApp):
    async def __call__(self, scope, receive, send):
        root_path = scope.get("root_path")
        if root_path and scope["path"].startswith(root_path):
            scope["path"] = scope["path"][len(root_path) :]
        return await super().__call__(scope, receive, send)


class Connection:

    def __init__(self):
        self.sio = socketio.AsyncServer(async_mode="asgi", cors_allowed_origins="*")
        self.app = SocketIoApp(socketio_server=self.sio, socketio_path="/socket.io")
        self._register_events()
        # Make the Socket.IO server globally available
        core.sio = self.sio

    def _register_events(self) -> None:

        @self.sio.event
        async def connect(sid: str, environ: Dict[str, Any]) -> None:
            print(f"🔌 Client connected: {sid}")

        @self.sio.event
        async def disconnect(sid: str) -> None:
            print(f"❌ Client disconnected: {sid}")

        """         
        @self.sio.on("invoke")  # type: ignore
        async def handle_invoke(sid: str, data: Dict[str, Any]) -> None:
            cmd = data.get("cmd")
            result_id = data.get("result_id")
            error_id = data.get("error_id")
            payload = data.get("payload", {})

            try:
                result = await registry.invoke(cmd, payload)  # type: ignore
                await self.sio.emit(
                    "invoke:result", {"id": result_id, "result": result}, to=sid
                )
            except Exception as e:
                await self.sio.emit(
                    "invoke:error", {"id": error_id, "error": str(e)}, to=sid
                ) """

        @self.sio.on("window_response")  # type: ignore
        async def window_response(sid: str, data: Dict[str, Any]) -> None:
            print(data)
            await handle_window_response(data)

        @self.sio.on("window_eventloop")  # type: ignore
        async def window_eventloop(sid: str, data: Dict[str, Any]) -> None:
            event = data.get("event")
            if event == "ipc.callback":
                return
            print(data)

        @self.sio.on("python:api")
        async def handle_api_request(sid: str, data: Dict[str, Any]):
            protocol = data.get("protocol")
            raw_payload = data.get("payload", {})
            result_id = None
            error_id = None
            if protocol == "rust:result:api":
                await handle_window_response(raw_payload)
            cmd = None
            py_payload = {}

            if "cmd" in raw_payload:
                cmd = raw_payload.get("cmd")
                result_id = raw_payload.get("result_id")
                error_id = raw_payload.get("error_id")
                py_payload = raw_payload.get("payload", {})
            else:
                py_payload = raw_payload

            try:

                full_payload = {"cmd": cmd, **py_payload} if cmd else py_payload
                raw_result = await ConnectionsProtocol.get_protocol(
                    protocol, full_payload
                )

                protocol = raw_result["protocol"]
                result = raw_result["result"]

                if result is not None and protocol == "pyinvoker":
                    await self.sio.emit(
                        "pyinvoke:result", {"id": result_id, "result": result}, to=sid
                    )
            except Exception as e:
                if protocol == "pyinvoker":
                    await self.sio.emit(
                        "pyinvoke:error", {"id": error_id, "error": str(e)}, to=sid
                    )


"""         @self.sio.on("menu_event")  # type: ignore
        async def window_response(sid: str, data: Dict[str, Any]) -> None:
            
            Beispiel-Daten:
            {
                'event': {
                    'api': 'menu',
                    'payload': {
                        'command_id': '69390e69-251b-4284-8ea1-a82824f6bb44',
                        'item_id': '1001',
                        'kind': 'menu_item'
                    }
                }
            }
            event = data.get("event")
            if not event:
                print("Kein Event in Payload gefunden!")
                return

            match event:
                case {"api": "menu", "payload": payload} if payload and "command_id" in payload:
                    command_id_str = payload["command_id"]
                    try:
                        command_id = UUID(command_id_str)
                    except ValueError:
                        print(f"Ungültige UUID: {command_id_str}")
                        return

                    # system_registry.trigger ist async → await nicht vergessen!
                    await system_registry.trigger(command_id)

                case {"api": api}:
                    print(f"Unbekanntes API: {api}")

                case _:
                    print("Ungültiges Event-Format:", event)
 """
