from typing import List, Literal, Optional

from ..app.runtime import request


class DialogHandel:
    async def show_message(
        self,
        title: str,
        content: Optional[str] = None,
        level: Optional[Literal["info", "warning", "error"]] = "info",
    ) -> None:

        return await request(
            "dialog.showMessage",
            {"title": title, "content": content, "level": level},
            scope=False,
        )

    async def pick_file(
        self,
        filters: Optional[List[str]] = None,
        start_dir: Optional[str] = None,
    ) -> Optional[str]:

        return await request(
            "dialog.pickFile", {"filters": filters, "start_dir": start_dir}, scope=False
        )

    async def pick_files(
        self,
        filters: Optional[List[str]] = None,
        start_dir: Optional[str] = None,
    ) -> Optional[List[str]]:

        return await request(
            "dialog.pickFiles",
            {"filters": filters, "start_dir": start_dir},
            scope=False,
        )

    async def pick_dir(
        self,
        start_dir: Optional[str] = None,
    ) -> Optional[str]:

        return await request("dialog.pickDir", {"start_dir": start_dir}, scope=False)

    async def pick_dirs(
        self,
        start_dir: Optional[str] = None,
    ) -> Optional[List[str]]:

        return await request("dialog.pickDirs", {"start_dir": start_dir}, scope=False)

    async def save_file(
        self,
        filters: Optional[List[str]] = None,
        start_dir: Optional[str] = None,
    ) -> Optional[str]:

        return await request(
            "dialog.saveFile", {"filters": filters, "start_dir": start_dir}, scope=False
        )
