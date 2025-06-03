from pathlib import Path
from typing import Any, Dict, Literal, Optional

from nicegui import app, run, ui

from pyframe import create_webview
from pyframe.model.models import AppOptions

START_DIR = str(Path(__file__).parent)


@app.get("/server_shutdown")
async def shutdown_server_endpoint():
    """
    HTTP GET endpoint to shut down the NiceGUI server cleanly.
    """
    if not app.is_stopped:
        app.shutdown()
        await app.stop()


def start_webview(
    host: str = "localhost",
    port: int = 8080,
    workers: Optional[int] = None,
    icon: Optional[str] = None,
    debug_devtools: Optional[bool] = None,
    debug_resource: Optional[str] = "resource",
    debug_entry: Optional[str] = None,
    fastapi_config: Optional[Dict[str, Any]] = None,
    enable_py_api: Optional[bool] = False,
    web_proto: Optional[str] = "http",
    menu_mode: Optional[Literal["menu", "tray", "menu_tray"]] = None,
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
    config = AppOptions(
        web_proto=web_proto,
        host=host,
        port=port,
        workers=workers,
        icon=icon,
        internal_api=enable_py_api,
        debug_devtools=debug_devtools,
        debug_resource=debug_resource,
        debug_entry=debug_entry,
        menu_mode=menu_mode,
    )
    create_webview(config.model_dump_json(indent=2, by_alias=True))


async def initialize_webview_window() -> None:
    """
    Asynchronously launch the WebView window as a CPU-bound task.
    """
    await run.cpu_bound(start_webview)


app.on_startup(initialize_webview_window)


# 2x2 Grid Layout for Cards
with ui.row().classes("grid grid-cols-2 grid-rows-2 gap-4 p-4"):
    with ui.card().classes(
        "flex flex-col items-center justify-center bg-white rounded-2xl shadow-xl p-4 transition hover:scale-105"
    ):
        ui.label(text="🪟 Native Capabilities with Tao").classes(
            "text-lg font-semibold text-gray-800"
        )
# Start the NiceGUI app (without showing the built-in browser window)
ui.run(show=False)
