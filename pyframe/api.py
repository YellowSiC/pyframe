import asyncio
from typing import Any, Dict, Optional

from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse

from . import background_tasks, core, outbox
from .model.models import AppOptions, SocketSettings
from .runtime import endless_state_loop


class FrameRESTAPI:
    def __init__(
        self, config: AppOptions, fastapi_config: Optional[Dict[str, Any]] = None
    ):
        self.config = config
        self.shutdown_event: Optional[asyncio.Event] = None
        self.app = FastAPI(**(fastapi_config or {}))
        self._configure_cors()
        self._create_routes()
        self.app.add_event_handler("startup", self._on_startup)
        self.app.add_event_handler("shutdown", self._on_shutdown)

    def _configure_cors(self) -> None:
        self.app.add_middleware(
            CORSMiddleware,
            allow_origins=["*"],
            allow_credentials=True,
            allow_methods=["*"],
            allow_headers=["*"],
        )

    def _create_routes(self) -> None:
        @self.app.get("/pyframe_socket_info")
        async def get_socket_settings(request: Request):
            prefix = request.headers.get(
                "X-Forwarded-Prefix", request.scope.get("root_path", "")
            )
            settings = self.config.socket_settings or SocketSettings()
            if not settings.path:
                settings.path = prefix or "/"
            settings_dict = settings.model_dump(by_alias=True)
            settings_dict["socketHost"] = self.config.host
            return JSONResponse(content=settings_dict)

        @self.app.get("/server_shutdown")
        async def shutdown():
            if self.shutdown_event:
                self.shutdown_event.set()
            return JSONResponse({"status": 200})

    async def _on_startup(self) -> None:
        core.state = core.State.STARTING
        core.loop = asyncio.get_running_loop()
        background_tasks.create(outbox.loop(), name="socket_loop")
        background_tasks.create(endless_state_loop(), name="scope_loop")
        core.state = core.State.STARTED

    async def _on_shutdown(self) -> None:
        core.state = core.State.STOPPED
