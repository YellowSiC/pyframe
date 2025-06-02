from typing import Optional, Tuple

from ..runtime import request


async def notification(
    summary: str,
    body: Optional[str] = None,
    app_id: Optional[str] = None,
    appname: Optional[str] = None,
    icon: Optional[str] = None,
    auto_icon: Optional[bool] = None,
    image_path: Optional[str] = None,
    sound_name: Optional[str] = None,
    subtitle: Optional[str] = None,
    timeout: Optional[int] = None,
    id: Optional[int] = None,
    action: Optional[Tuple[str, str]] = None,
) -> None:
    """Leerer Platzhalter, der einen asynchronen Request sendet."""

    payload = {
        "summary": summary,
        "body": body,
        "app_id": app_id,
        "appname": appname,
        "icon": icon,
        "auto_icon": auto_icon,
        "image_path": image_path,
        "sound_name": sound_name,
        "subtitle": subtitle,
        "timeout": timeout,
        "id": id,
        "action": action,
    }

    await request("controlcenter.notification", payload, scope=False)
