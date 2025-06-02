from typing import Optional, List, Dict
from ..runtime import request

class ResourceAPI:
    """
    Platzhalter-Stub für Resource-API.
    Alle Methoden geben Dummy-Werte zurück oder tun nichts.
    """

    async def watch(self, path: str, callback_url: str, max_events: int, timeout_secs: int) -> None:
        payload = {
            "path": path,
            "callback_url": callback_url,
            "max_events": max_events,
            "timeout_secs": timeout_secs
        }
        return await request("resource.watch", payload, scope=False)

    async def translate(self, lang_path: str, key: str) -> str:
        payload = {"lang_path": lang_path, "key": key}
        return await request("resource.translate", payload, scope=False)

    async def bundle(self, paths: List[str]) -> Dict[str, str]:
        payload = {"paths": paths}
        return await request("resource.bundle", payload, scope=False)

    async def thumbnail(self, path: str, max_size: int) -> str:
        payload = {"path": path, "max_size": max_size}
        return await request("resource.thumbnail", payload, scope=False)

    async def exists(self, path: str) -> bool:
        payload = {"path": path}
        return await request("resource.exists", payload, scope=False)

    async def read(self, path: str, encode: Optional[str] = None) -> str:
        payload = {"path": path, "encode": encode}
        return await request("resource.read", payload, scope=False)

    async def extract(self, from_path: str, to_path: str) -> None:
        payload = {"from": from_path, "to": to_path}
        return await request("resource.extract", payload, scope=False)

    async def metadata(self, path: str) -> str:
        payload = {"path": path}
        return await request("resource.metadata", payload, scope=False)

    async def list(self, dir: str) -> List[str]:
        payload = {"dir": dir}
        return await request("resource.list", payload, scope=False)

    async def list_recursive(self, path: str) -> List[str]:
        payload = {"path": path}
        return await request("resource.list_recursive", payload, scope=False)

    async def delete(self, path: str) -> None:
        payload = {"path": path}
        return await request("resource.delete", payload, scope=False)

    async def copy(self, from_path: str, to_path: str) -> None:
        payload = {"from": from_path, "to": to_path}
        return await request("resource.copy", payload, scope=False)

    async def read_bytes(self, path: str) -> str:
        payload = {"path": path}
        return await request("resource.read_bytes", payload, scope=False)

    async def read_json(self, path: str) -> str:
        payload = {"path": path}
        return await request("resource.read_json", payload, scope=False)

    async def mime_type(self, path: str) -> str:
        payload = {"path": path}
        return await request("resource.mime_type", payload, scope=False)

    async def hash(self, path: str) -> str:
        payload = {"path": path}
        return await request("resource.hash", payload, scope=False)
