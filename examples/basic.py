"""
Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
nuitka --standalone --output-dir=dist --include-data-dir=c:\\Users\\MalekAli\\Desktop\\malek\\resource=resource basic.py
"""

import sys
from pathlib import Path

import pydantic

from pyframe import Frame, Menu, PyFrame, SubMenu, WindowsWindowConfig, command
from pyframe.handler.notify import notification
from pyframe.model.models import AcceleratorCode, AcceleratorModifier


class Person(pydantic.BaseModel):
    name: str


async def menuitem():
    print("Menu has been called from Rust!")


submenu = SubMenu(title="Malek", enabled=True)
submenu.add_menu_item(
    text="File" + " ",
    enabled=True,
    modifier=AcceleratorModifier.alt,
    key=AcceleratorCode.key_c,
    command=menuitem,
)
menu = Menu()
menu.add_menu_item(
    text="File" + " ",
    enabled=True,
    modifier=AcceleratorModifier.alt,
    key=AcceleratorCode.key_c,
    command=menuitem,
)
menu.add_submenu(submenus=[submenu])
menu.add_system_tray(title="Malek", icon=Path(__file__).parent / "resource/icon.png")

config = Frame()
config.window_entry(path="index.html")  # this does not have to be activated in NiceGui
config.window_inner_size(size=(900, 700))
app = PyFrame(
    debug_resource="resource",
    web_proto="pyframe",  # set web_proto to "https or http" so that the internal web protocol is not activated
    debug_devtools=True,
    menu_mode="tray",
    enable_py_api=True,
    # debug_entry="http://localhost:8080",
)

app.initial_window(window=config)


app.set_platform_config(windows=WindowsWindowConfig(taskbar_icon="icon.png"))
app.set_frame_menu(menu)


@command
async def greet(name: Person) -> pydantic.RootModel[str]:
    d = await notification(
        summary="Malek", icon=str(Path(__file__).parent / "resource/icon.png")
    )

    return pydantic.RootModel[str](
        f"Hello {name.name}! You've been greeted from Python {sys.winver}!"
    )


# If this script is run directly (e.g., with `python examples/basic.py`):
if __name__ == "__main__":
    # Start the WebView application (opens a window with the embedded WebView)
    app.start()
