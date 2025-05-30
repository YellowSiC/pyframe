"""
pyframe.py

Main PyFrame class to manage the runtime with FastAPI (REST API),
Socket.IO (WebSocket server), and a WebView process.
"""

import asyncio
import multiprocessing
import signal
import sys
import threading
import time
from typing import Any, Dict, List, Optional

import uvicorn

from ..api import FrameRESTAPI
from ..config import WindowConfigurator
from ..connection import Connection
from ..model.models import (AppOptions, CheckMenuItem, IconMenuItem,
                            LinuxWindowConfig, MacOSWindowConfig, MenuFrame,
                            MenuItem, PredefinedMenuItem, SocketSettings,
                            Submenu, SystemTray, WindowsWindowConfig)
from ..utils import suppress_stderr
from .runtime import run_webview


class PyFrame:
    """
    Main class to manage the PyFrame runtime, combining FastAPI (REST API),
    Socket.IO (WebSocket server), and the webview process.
    """

    def __init__(
        self,
        host: str = "localhost",
        port: int = 8080,
        workers: Optional[int] = None,
        icon: Optional[str] = None,
        debug_devtools: Optional[bool] = None,
        debug_resource: Optional[str] = None,
        debug_entry: Optional[str] = None,
        fastapi_config: Optional[Dict[str, Any]] = None,
        enable_py_api: Optional[bool] = True,
        web_proto: Optional[str] = None,
    ):
        """
        Initialize the PyFrame application and its components.

        Args:
            host: The host address for the API server.
            port: The port for the API server.
            workers: Number of workers for the API server.
            icon: Path to the window icon.
            debug_devtools: Whether to enable DevTools in debug mode.
            debug_resource: Optional debug resource path.
            debug_entry: Optional debug entry file.
            fastapi_config: Additional FastAPI configuration.
        """
        self.host = host
        self.port = port
        self.config = AppOptions(
            web_proto=web_proto,
            host=host,
            port=port,
            workers=workers,
            icon=icon,
            internal_api=enable_py_api,
            debug_devtools=debug_devtools,
            debug_resource=debug_resource,
            debug_entry=debug_entry,
        )

        self.shutdown_event = threading.Event()
        self.system_shutdown_event = asyncio.Event()
        self.system_startup_event = asyncio.Event()

        # Create FastAPI (REST API) and Socket.IO (WebSocket) servers
        self.fastapi_app = FrameRESTAPI(self.config, fastapi_config)
        self.socketio_app = Connection()

        # Assign a compatible shutdown event (if FrameRESTAPI expects threading.Event)
        # If FrameRESTAPI expects asyncio.Event, pass system_shutdown_event instead.
        self.fastapi_app.shutdown_event = self.shutdown_event  # type: ignore

        # Mount the Socket.IO app within FastAPI
        self.fastapi_app.app.mount("/_pyframe_ws/", self.socketio_app.app)

        self.api_thread: Optional[threading.Thread] = None
        self.webview_process: Optional[multiprocessing.Process] = None

    def configure_socketio_on_window(self, **kwargs: Any) -> None:
        """
        Configure Socket.IO settings to be passed to the window.

        Args:
            **kwargs: Additional socket settings.
        """
        self.config.socket_settings = SocketSettings(**kwargs)

    def initial_window(self, window: WindowConfigurator) -> None:
        """
        Set the initial window configuration.

        Args:
            window: The window configurator to use.
        """
        if window:
            self.config.window = window.build()

    def set_platform_config(
        self,
        windows: Optional[WindowsWindowConfig] = None,
        linux: Optional[LinuxWindowConfig] = None,
        macos: Optional[MacOSWindowConfig] = None,
    ) -> None:
        """
        Set platform-specific configurations.

        Args:
            windows: Windows-specific window config.
            linux: Linux-specific window config.
            macos: macOS-specific window config.
        """
        if windows:
            self.config.windows_extra = windows
        if linux:
            self.config.linux_extra = linux
        if macos:
            self.config.macos_extra = macos

    def start_fastapi(self) -> None:
        """
        Start the FastAPI server in a separate thread.
        """

        async def run_server() -> None:
            config = uvicorn.Config(
                app=self.fastapi_app.app,
                host=self.host,
                port=self.port,
                log_config=None,
            )
            server = uvicorn.Server(config)
            await server.serve()

        self.api_thread = threading.Thread(
            target=asyncio.run, args=(run_server(),), daemon=True
        )
        self.api_thread.start()

    def start_webview(self) -> None:
        """
        Start the webview process in a separate process.
        """
        config_json = self.config.model_dump_json(indent=2, by_alias=True)
        self.webview_process = multiprocessing.Process(
            target=run_webview, args=(config_json,)
        )
        self.webview_process.start()

    def stop(self) -> None:
        """
        Stop the webview process and exit the application.
        """
        if self.webview_process and self.webview_process.is_alive():
            self.webview_process.terminate()
            self.webview_process.join()
        sys.exit(0)

    def start(self) -> None:
        """
        Start the entire PyFrame system, including the API server and the webview.

        Handles graceful shutdown upon receiving SIGINT or SIGTERM signals.
        """
        with suppress_stderr():
            self.start_fastapi()
            self.start_webview()

            def handle_signal(sig: int, frame: Optional[Any]) -> None:
                self.shutdown_event.set()

            signal.signal(signal.SIGINT, handle_signal)
            signal.signal(signal.SIGTERM, handle_signal)

            try:
                while not self.shutdown_event.is_set():
                    if self.webview_process and self.webview_process.is_alive():
                        self.webview_process.join(timeout=0.5)
                    else:
                        print("[WARN] webview_process not alive. Waiting...")
                        time.sleep(0.5)
            except KeyboardInterrupt:
                print("Keyboard interrupt detected. Exiting...")
                self.shutdown_event.set()
            print("shutdown detected. Exiting...")
            self.stop()
