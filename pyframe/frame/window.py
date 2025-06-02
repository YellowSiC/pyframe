from typing import List, Optional, Tuple

from ..model.models import (
    CheckMenuItem,
    FrameBackgroundThrottlingPolicy,
    IconMenuItem,
    MenuFrame,
    MenuItem,
    PredefinedMenuItem,
    Submenu,
    WindowConfig,
)


class Frame:
    def __init__(self):
        self.__config = WindowConfig()

    def window_inner_size(self, size: Tuple[int, int]):
        self.__config.window_inner_size = size
        return self

    def window_min_inner_size(self, size: Tuple[int, int]):
        self.__config.window_min_inner_size = size
        return self

    def window_max_inner_size(self, size: Tuple[int, int]):
        self.__config.window_max_inner_size = size
        return self

    def window_position(self, position: Tuple[int, int]):
        self.__config.window_position = position
        return self

    def window_resizable(self, value: bool):
        self.__config.window_resizable = value
        return self

    def window_minimizable(self, value: bool):
        self.__config.window_minimizable = value
        return self

    def window_maximizable(self, value: bool):
        self.__config.window_maximizable = value
        return self

    def window_closable(self, value: bool):
        self.__config.window_closable = value
        return self

    def window_entry(self, path: str):
        self.__config.entry = path
        return self

    def window_title(self, title: str):
        self.__config.window_title = title
        return self

    def window_fullscreen(self, value: bool):
        self.__config.window_fullscreen = value
        return self

    def window_maximized(self, value: bool):
        self.__config.window_maximized = value
        return self

    def window_visible(self, value: bool):
        self.__config.window_visible = value
        return self

    def window_transparent(self, value: bool):
        self.__config.window_transparent = value
        return self

    def window_decorations(self, value: bool):
        self.__config.window_decorations = value
        return self

    def window_always_on_bottom(self, value: bool):
        self.__config.window_always_on_bottom = value
        return self

    def window_always_on_top(self, value: bool):
        self.__config.window_always_on_top = value
        return self

    def window_window_icon(self, path: str):
        self.__config.window_window_icon = path
        return self

    def window_theme(self, theme: str):
        self.__config.window_theme = theme
        return self

    def window_focused(self, value: bool):
        self.__config.window_focused = value
        return self

    def window_content_protection(self, value: bool):
        self.__config.window_content_protection = value
        return self

    def window_visible_on_all_workspaces(self, value: bool):
        self.__config.window_visible_on_all_workspaces = value
        return self

    def window_background_color(self, rgba: Tuple[int, int, int, int]):
        self.__config.window_background_color = rgba
        return self

    def webview_context_id(self, ctx_id: str):
        self.__config.webview_context_id = ctx_id
        return self

    def webview_transparent(self, value: bool):
        self.__config.webview_transparent = value
        return self

    def webview_background_color(self, rgba: Tuple[int, int, int, int]):
        self.__config.webview_background_color = rgba
        return self

    def webview_visible(self, value: bool):
        self.__config.webview_visible = value
        return self

    def webview_autoplay(self, value: bool):
        self.__config.webview_autoplay = value
        return self

    def webview_initialization_scripts(self, scripts: List[str]):
        self.__config.webview_initialization_scripts = scripts
        return self

    def webview_initialization_main_only(self, scripts: List[Tuple[str, bool]]):
        self.__config.webview_initialization_main_only = scripts
        return self

    def webview_headers(self, headers: dict[str, str]):
        self.__config.webview_headers = headers
        return self

    def webview_user_agent(self, ua: str):
        self.__config.webview_user_agent = ua
        return self

    def webview_devtools(self, value: bool):
        self.__config.webview_devtools = value
        return self

    def webview_hotkeys_zoom(self, value: bool):
        self.__config.webview_hotkeys_zoom = value
        return self

    def webview_clipboard(self, value: bool):
        self.__config.webview_clipboard = value
        return self

    def webview_incognito(self, value: bool):
        self.__config.webview_incognito = value
        return self

    def webview_focused(self, value: bool):
        self.__config.webview_focused = value
        return self

    def webview_bounds(self, bounds: Tuple[int, int, int, int]):
        self.__config.webview_bounds = bounds
        return self

    def webview_javascript_disabled(self, value: bool):
        self.__config.webview_javascript_disabled = value
        return self

    def webview_accept_first_mouse(self, value: bool):
        self.__config.webview_accept_first_mouse = value
        return self

    def webview_back_forward_navigation_gestures(self, value: bool):
        self.__config.webview_back_forward_navigation_gestures = value
        return self

    def webview_background_throttling(self, value: FrameBackgroundThrottlingPolicy):
        self.__config.webview_background_throttling = value
        return self

    def webview_proxy_config(self, config: dict):
        self.__config.webview_proxy_config = config
        return self

    def build(self) -> WindowConfig:
        return self.__config
