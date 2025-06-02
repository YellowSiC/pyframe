# Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

from typing import Any, Dict, List, Optional

from ..runtime import request


class MonitorAPI:

    async def list(self) -> List[Dict[str, Any]]:
        return await request("monitor.list", {}, scope=False)

    async def current(self) -> Optional[Dict[str, Any]]:
        return await request("monitor.current", {}, scope=False)

    async def primary(self) -> Optional[Dict[str, Any]]:
        return await request("monitor.primary", {}, scope=False)

    async def from_point(self, x: float, y: float) -> Optional[Dict[str, Any]]:
        return await request("monitor.fromPoint", {"x": x, "y": y}, scope=False)
