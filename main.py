"""
Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT

"""

import sys
# Import the main PyFrame class
from pathlib import Path

import pydantic

from pyframe import PyFrame, WindowConfigurator, WindowsWindowConfig, command
from pyframe.model.models import AcceleratorCode, AcceleratorModifier, MenuItem


class Person(pydantic.BaseModel):
    name: str



async def menu():
    print("Menu has been called from Rust!")




config = WindowConfigurator()
config.window_entry(path="index.html")
# config.window_decorations(value=False)
config.window_inner_size(size=(900, 700))
config.window_menu(
    menu_items=[
        MenuItem(
            text="File" + " ",
            enabled=True,
            modifier=AcceleratorModifier.control,
            key=AcceleratorCode.key_c,
            command=menu,
        )
    ]
)
# config.window_window_icon(path=str(Path(__file__).parent / "resource/icon.png"))
# Create an instance of the PyFrame application
app = PyFrame(
    # Path to the static resources folder
    # This is the **entry point** for all static files (HTML, CSS, JS, etc.).
    # This parameter is **mandatory** because PyFrame has an internal resource_manager.
    # If you don’t specify a static folder, you will get an error!
    debug_resource="resource",
    # This is the URL of your **development server**.
    # For example, for frontend frameworks like React, Next.js, Vue.js, Angular, Preact, Svelte, etc.
    # It allows PyFrame to connect to your dev server in "debug mode"
    # to enable things like hot reloading and HMR.
    # debug_entry="https://nicegui.io",
    web_proto="pyframe",
    debug_devtools=True
)

app.initial_window(window=config)


app.set_platform_config(windows=WindowsWindowConfig(taskbar_icon="icon.png"))


@command
async def greet(name: Person) -> pydantic.RootModel[str]:
    return pydantic.RootModel[str](
        f"Hello {name.name}! You've been greeted from Python {sys.winver}!"
    )


# If this script is run directly (e.g., with `python examples/basic.py`):
if __name__ == "__main__":
    # Start the WebView application (opens a window with the embedded WebView)
    app.start()




