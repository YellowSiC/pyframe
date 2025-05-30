from typing import Any, Optional

from ..app.runtime import request


class WindowHandel:

    async def current(self) -> Any:
        return await request("window.current", {})

    async def close(self, id: Optional[int] = None) -> Any:
        return await request("window.close", {"id": id}, scope=False)

    async def list(self) -> Any:
        return await request("window.list", {})

    async def send_message(self, message: str, id: int) -> Any:
        return await request(
            "window.sendMessage", {"message": message, "id": id}, scope=False
        )

    async def set_menu(
        self, options: Optional[dict] = None, id: Optional[int] = None
    ) -> Any:
        return await request(
            "window.setMenu", {"options": options, "id": id}, scope=False
        )

    async def hide_menu(self, id: Optional[int] = None) -> Any:
        return await request("window.hideMenu", {"id": id}, scope=False)

    async def show_menu(self, id: Optional[int] = None) -> Any:
        return await request("window.showMenu", {"id": id}, scope=False)

    async def is_menu_visible(self, id: Optional[int] = None) -> Any:
        return await request("window.isMenuVisible", {"id": id})

    async def scale_factor(self, id: Optional[int] = None) -> Any:
        return await request("window.scaleFactor", {"id": id})

    async def inner_position(self, id: Optional[int] = None) -> Any:
        return await request("window.innerPosition", {"id": id})

    async def outer_position(self, id: Optional[int] = None) -> Any:
        return await request("window.outerPosition", {"id": id})

    async def set_outer_position(self, position: dict, id: Optional[int] = None) -> Any:
        return await request(
            "window.setOuterPosition", {"position": position, "id": id}, scope=False
        )

    async def inner_size(self, id: Optional[int] = None) -> Any:
        return await request("window.innerSize", {"id": id})

    async def set_inner_size(self, size: dict, id: Optional[int] = None) -> Any:
        return await request(
            "window.setInnerSize", {"size": size, "id": id}, scope=False
        )

    async def outer_size(self, id: Optional[int] = None) -> Any:
        return await request("window.outerSize", {"id": id})

    async def set_min_inner_size(self, size: dict, id: Optional[int] = None) -> Any:
        return await request(
            "window.setMinInnerSize", {"size": size, "id": id}, scope=False
        )

    async def set_max_inner_size(self, size: dict, id: Optional[int] = None) -> Any:
        return await request(
            "window.setMaxInnerSize", {"size": size, "id": id}, scope=False
        )

    async def set_title(self, title: str, id: Optional[int] = None) -> Any:
        return await request("window.setTitle", {"title": title, "id": id}, scope=False)

    async def title(self, id: Optional[int] = None) -> Any:
        return await request("window.title", {"id": id})

    async def is_visible(self, id: Optional[int] = None) -> Any:
        return await request("window.isVisible", {"id": id})

    async def set_visible(self, visible: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setVisible", {"visible": visible, "id": id}, scope=False
        )

    async def is_focused(self, id: Optional[int] = None) -> Any:
        return await request("window.isFocused", {"id": id})

    async def set_focus(self, id: Optional[int] = None) -> Any:
        return await request("window.setFocus", {"id": id}, scope=False)

    async def is_resizable(self, id: Optional[int] = None) -> Any:
        return await request("window.isResizable", {"id": id})

    async def set_resizable(self, resizable: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setResizable", {"resizable": resizable, "id": id}, scope=False
        )

    async def is_minimizable(self, id: Optional[int] = None) -> Any:
        return await request("window.isMinimizable", {"id": id})

    async def set_minimizable(self, minimizable: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setMinimizable", {"minimizable": minimizable, "id": id}, scope=False
        )

    async def is_maximizable(self, id: Optional[int] = None) -> Any:
        return await request("window.isMaximizable", {"id": id})

    async def set_maximizable(self, maximizable: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setMaximizable", {"maximizable": maximizable, "id": id}, scope=False
        )

    async def is_closable(self, id: Optional[int] = None) -> Any:
        return await request("window.isClosable", {"id": id})

    async def set_closable(self, closable: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setClosable", {"closable": closable, "id": id}, scope=False
        )

    async def is_minimized(self, id: Optional[int] = None) -> Any:
        return await request("window.isMinimized", {"id": id})

    async def set_minimized(self, minimized: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setMinimized", {"minimized": minimized, "id": id}, scope=False
        )

    async def is_maximized(self, id: Optional[int] = None) -> Any:
        return await request("window.isMaximized", {"id": id})

    async def set_maximized(self, maximized: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setMaximized", {"maximized": maximized, "id": id}, scope=False
        )

    async def decorated(self, id: Optional[int] = None) -> Any:
        return await request("window.Decorated", {"id": id})

    async def set_decorated(self, decorated: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setDecorated", {"decorated": decorated, "id": id}, scope=False
        )

    async def fullscreen(self, id: Optional[int] = None) -> Any:
        return await request("window.fullscreen", {"id": id})

    async def set_fullscreen(
        self,
        is_fullscreen: bool,
        monitor_name: Optional[str] = None,
        id: Optional[int] = None,
    ) -> Any:
        return await request(
            "window.setFullscreen",
            {"is_fullscreen": is_fullscreen, "monitor_name": monitor_name, "id": id},
            scope=False,
        )

    async def set_always_on_top(
        self, always_on_top: bool, id: Optional[int] = None
    ) -> Any:
        return await request(
            "window.setAlwaysOnTop",
            {"always_on_top": always_on_top, "id": id},
            scope=False,
        )

    async def set_always_on_bottom(
        self, always_on_bottom: bool, id: Optional[int] = None
    ) -> Any:
        return await request(
            "window.setAlwaysOnBottom",
            {"always_on_bottom": always_on_bottom, "id": id},
            scope=False,
        )

    async def request_user_attention(self, level: str, id: Optional[int] = None) -> Any:
        return await request(
            "window.requestUserAttention", {"level": level, "id": id}, scope=False
        )

    async def set_content_protection(
        self, enabled: bool, id: Optional[int] = None
    ) -> Any:
        return await request(
            "window.setContentProtection", {"enabled": enabled, "id": id}, scope=False
        )

    async def set_visible_on_all_workspaces(
        self, visible: bool, id: Optional[int] = None
    ) -> Any:
        return await request(
            "window.setVisibleOnAllWorkspaces",
            {"visible": visible, "id": id},
            scope=False,
        )

    async def set_cursor_icon(self, icon: str, id: Optional[int] = None) -> Any:
        return await request(
            "window.setCursorIcon", {"icon": icon, "id": id}, scope=False
        )

    async def cursor_position(self, id: Optional[int] = None) -> Any:
        return await request("window.cursorPosition", {"id": id})

    async def set_cursor_position(
        self, position: dict, id: Optional[int] = None
    ) -> Any:
        return await request(
            "window.setCursorPosition", {"position": position, "id": id}, scope=False
        )

    async def set_cursor_grab(self, grab: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setCursorGrab", {"grab": grab, "id": id}, scope=False
        )

    async def set_cursor_visible(self, visible: bool, id: Optional[int] = None) -> Any:
        return await request(
            "window.setCursorVisible", {"visible": visible, "id": id}, scope=False
        )

    async def drag_window(self, id: Optional[int] = None) -> Any:
        return await request("window.dragWindow", {"id": id})

    async def set_ignore_cursor_events(
        self, ignore: bool, id: Optional[int] = None
    ) -> Any:
        return await request(
            "window.setIgnoreCursorEvents", {"ignore": ignore, "id": id}, scope=False
        )

    async def theme(self, id: Optional[int] = None) -> Any:
        return await request("window.theme", {"id": id})

    async def block_close_requested(
        self, blocked: bool, id: Optional[int] = None
    ) -> Any:
        return await request(
            "window.blockCloseRequested", {"blocked": blocked, "id": id}, scope=False
        )
