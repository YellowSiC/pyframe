# Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

from typing import Optional, Tuple
from ..runtime import request

class ExtraAPI:

    async def get_active_window_id(self) -> Optional[str]:
        return await request("extra.getActiveWindowId",{},scope=False)

    async def focus_by_window_id(self, id_string: str) -> bool:
        return await request("extra.focusByWindowId",{"id_string":id_string},scope=False)

    async def hide_application(self) -> None:
        return await request("extra.hideApplication",{},scope=False)

    async def show_application(self) -> None:
        return await request("extra.showApplication",{},scope=False)

    async def hide_other_applications(self) -> None:
        return await request("extra.hideOtherApplications",{},scope=False)

    async def set_activation_policy(self, policy: str) -> None:
        return await request("extra.setActivationPolicy",{"policy":policy},scope=False)
