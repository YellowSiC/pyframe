from typing import List, Optional, Tuple

from ..runtime import request


class ShortcutAPI:
    """
    Platzhalter-Stub für Shortcut-API.
    Diese Methoden senden ihre Parameter korrekt an die API.
    """

    async def register(self, shortcut: dict, window_id: Optional[int] = None) -> int:
        """
        Registriert ein Tastenkürzel.
        """
        payload = {"shortcut": shortcut, "window_id": window_id}
        return await request("shortcut.register", payload, scope=False)

    async def unregister(self, id: int, window_id: Optional[int] = None) -> None:
        """
        Hebt die Registrierung eines Shortcut-IDs auf.
        """
        payload = {"id": id, "window_id": window_id}
        return await request("shortcut.unregister", payload, scope=False)

    async def unregister_all(self, window_id: Optional[int] = None) -> None:
        """
        Hebt alle Shortcuts für ein Fenster auf.
        """
        payload = {"window_id": window_id}
        return await request("shortcut.unregisterAll", payload, scope=False)

    async def list(self, window_id: Optional[int] = None) -> List[Tuple[int, str]]:
        """
        Listet alle registrierten Shortcuts auf.
        """
        payload = {"window_id": window_id}
        return await request("shortcut.list", payload, scope=False)
