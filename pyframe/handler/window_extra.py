from typing import Optional
from ..runtime import request

class WindowExtraAPI:
    """
    Platzhalter-Stub für WindowExtra-API (Windows/macOS).
    """

    # Windows-spezifische Methoden
    async def set_enable(self, enabled: bool, id: Optional[int] = None) -> None:
        payload = {"enabled": enabled, "id": id}
        return await request("windowExtra.setEnable", payload, scope=False)

    async def set_taskbar_icon(self, taskbar_icon: str, id: Optional[int] = None) -> None:
        payload = {"taskbar_icon": taskbar_icon, "id": id}
        return await request("windowExtra.setTaskbarIcon", payload, scope=False)

    async def theme(self, id: Optional[int] = None) -> str:
        payload = {"id": id}
        return await request("windowExtra.theme", payload, scope=False)

    async def reset_dead_keys(self, id: Optional[int] = None) -> None:
        payload = {"id": id}
        return await request("windowExtra.resetDeadKeys", payload, scope=False)

    async def begin_resize_drag(self, edge: int, button: int, x: int, y: int, id: Optional[int] = None) -> None:
        payload = {"edge": edge, "button": button, "x": x, "y": y, "id": id}
        return await request("windowExtra.beginResizeDrag", payload, scope=False)

    async def set_skip_taskbar(self, skip: bool, id: Optional[int] = None) -> None:
        payload = {"skip": skip, "id": id}
        return await request("windowExtra.setSkipTaskbar", payload, scope=False)

    async def set_undecorated_shadow(self, shadow: bool, id: Optional[int] = None) -> None:
        payload = {"shadow": shadow, "id": id}
        return await request("windowExtra.setUndecoratedShadow", payload, scope=False)

    # macOS-spezifische Methoden
    async def simple_fullscreen(self, id: Optional[int] = None) -> bool:
        payload = {"id": id}
        return await request("windowExtra.simpleFullscreen", payload, scope=False)

    async def set_simple_fullscreen(self, fullscreen: bool, id: Optional[int] = None) -> bool:
        payload = {"fullscreen": fullscreen, "id": id}
        return await request("windowExtra.setSimpleFullscreen", payload, scope=False)

    async def has_shadow(self, id: Optional[int] = None) -> bool:
        payload = {"id": id}
        return await request("windowExtra.hasShadow", payload, scope=False)

    async def set_has_shadow(self, has_shadow: bool, id: Optional[int] = None) -> None:
        payload = {"has_shadow": has_shadow, "id": id}
        return await request("windowExtra.setHasShadow", payload, scope=False)

    async def set_is_document_edited(self, edited: bool, id: Optional[int] = None) -> None:
        payload = {"edited": edited, "id": id}
        return await request("windowExtra.setIsDocumentEdited", payload, scope=False)

    async def is_document_edited(self, id: Optional[int] = None) -> bool:
        payload = {"id": id}
        return await request("windowExtra.isDocumentEdited", payload, scope=False)

    async def set_allows_automatic_window_tabbing(self, enabled: bool, id: Optional[int] = None) -> None:
        payload = {"enabled": enabled, "id": id}
        return await request("windowExtra.setAllowsAutomaticWindowTabbing", payload, scope=False)

    async def allows_automatic_window_tabbing(self, id: Optional[int] = None) -> bool:
        payload = {"id": id}
        return await request("windowExtra.allowsAutomaticWindowTabbing", payload, scope=False)

    async def set_tabbing_identifier(self, identifier: str, id: Optional[int] = None) -> None:
        payload = {"identifier": identifier, "id": id}
        return await request("windowExtra.setTabbingIdentifier", payload, scope=False)

    async def tabbing_identifier(self, id: Optional[int] = None) -> str:
        payload = {"id": id}
        return await request("windowExtra.tabbingIdentifier", payload, scope=False)
