from typing import Any, Optional

from ..runtime import request


class WebviewHandel:
    async def is_devtools_open(self) -> Any:
        return await request("webview.isDevtoolsOpen", {}, scope=False)

    async def open_devtools(self) -> Any:
        return await request("webview.openDevtools", {}, scope=False)

    async def close_devtools(self) -> Any:
        return await request("webview.closeDevtools", {}, scope=False)

    async def bounds(self) -> Any:
        return await request("webview.bounds", {}, scope=False)

    async def clear_all_browsing_data(self) -> Any:
        return await request("webview.clearAllBrowsingData", {}, scope=False)

    async def cookies(self) -> Any:
        return await request("webview.cookies", {}, scope=False)

    async def cookies_for_url(self, url: str) -> Any:
        return await request("webview.cookiesForUrl", {"url": url}, scope=False)

    async def evaluate_script(self, code: str) -> Any:
        return await request("webview.evaluateScript", {"code": code}, scope=False)

    async def focus(self) -> Any:
        return await request("webview.focus", {}, scope=False)

    async def load_html(self, code: str) -> Any:
        return await request("webview.loadHtml", {"code": code}, scope=False)

    async def load_url(self, url: str) -> Any:
        return await request("webview.loadUrl", {"url": url}, scope=False)

    async def zoom(self, scale: float) -> Any:
        return await request("webview.zoom", {"scale": scale}, scope=False)

    async def print(self) -> Any:
        return await request("webview.print", {}, scope=False)

    async def reload(self) -> Any:
        return await request("webview.reload", {}, scope=False)

    async def url(self) -> Any:
        return await request("webview.url", {}, scope=False)

    async def set_background_color(
        self, r: int, g: int, b: int, a: Optional[int] = 255
    ) -> Any:
        return await request(
            "webview.setBackgroundColor", {"r": r, "g": g, "b": b, "a": a}, scope=False
        )

    async def set_bounds(self, x: int, y: int, width: int, height: int) -> Any:
        return await request(
            "webview.setBounds",
            {"x": x, "y": y, "width": width, "height": height},
            scope=False,
        )

    async def visible(self, visible: bool) -> Any:
        return await request("webview.visible", {"visible": visible}, scope=False)

    async def load_url_with_headers(
        self, url: str, headers_json: Optional[dict] = None
    ) -> Any:
        return await request(
            "webview.loadUrlWithHeaders",
            {"url": url, "headers_json": headers_json},
            scope=False,
        )

    async def webview_id(self) -> Any:
        return await request("webview.webviewId", {}, scope=False)

    async def focus_parent(self) -> Any:
        return await request("webview.focusParent", {}, scope=False)

    async def reparent(self, hwnd: int) -> Any:
        return await request("webview.reparent", {"hwnd": hwnd}, scope=False)

    async def set_memory_usage_level(self, level: str) -> Any:
        return await request(
            "webview.setMemoryUsageLevel", {"level": level}, scope=False
        )

    async def cotroller(self) -> Any:
        return await request("webview.controller", {}, scope=False)

    async def set_theme(self, theme: bool) -> Any:
        return await request("webview.setTheme", {"theme": theme}, scope=False)

    async def base_url(self) -> Any:
        return await request("webview.baseUrl", {}, scope=False)

    async def base_filesystem_url(self) -> Any:
        return await request("webview.baseFileSystemUrl", {}, scope=False)
