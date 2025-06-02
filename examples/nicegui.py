from typing import Any, Dict, Optional
from pyframe.model.models import AppOptions
from pyframe.runtime import run_webview
from nicegui import app, run, ui
from pathlib import Path


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
    host: Optional[str] = "localhost",
    port: Optional[int] = 8080,
    workers: Optional[int] = None,
    icon: Optional[str] = None,
    debug_devtools: Optional[bool] = None,
    debug_resource: Optional[str] = None,
    debug_entry: Optional[str] = None,
    web_proto='http'
):
    """
    Launch the PyFrame-based WebView with the given basic configuration.
    """
    config = AppOptions(
        host=host,
        port=port,
        workers=workers,
        icon=icon,
        web_proto = web_proto,
        debug_devtools=debug_devtools,
        debug_resource="resource",
        debug_entry="http://localhost:8080",
        internal_api=False
    
    )
    run_webview(config.model_dump_json(indent=2, by_alias=True))


async def initialize_webview_window() -> None:
    """
    Asynchronously launch the WebView window as a CPU-bound task.
    """
    await run.cpu_bound(start_webview)

app.on_startup(initialize_webview_window)

# Start the NiceGUI app (without showing the built-in browser window)
ui.run(show=False)