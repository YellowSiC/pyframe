from __future__ import annotations

import asyncio
from collections import deque
from typing import Any, Deque, Optional, Tuple

from . import core

ClientId = str
MessageType = str
Message = Tuple[Optional[ClientId], MessageType, Any]


message_queue: Deque[Message] = deque()
message_dequeue: Deque[Tuple[MessageType, Any, Optional[str]]] = deque()


def emit_event(
    event: MessageType, data: Any, target_id: Optional[ClientId] = None
) -> None:
    message_queue.append((target_id, event, data))


def on_event(event: MessageType, handler, namespace: Optional[str] = None) -> None:

    async def handle_message(*args, **kwargs):
        message_data = {}
        if len(args) > 0:
            message_data["sid"] = args[0]
            message_data["data"] = args[1]
        if kwargs:
            message_data.update(kwargs)

        await handler(message_data)

    message_dequeue.append((event, handle_message, namespace))


async def _emit(
    event: MessageType, data: Any, target_id: Optional[ClientId] = None
) -> None:
    await core.sio.emit(event, data, room=target_id)


async def _on(event: MessageType, handler, namespace: Optional[str] = None) -> None:

    @core.sio.on(event, namespace)  # type: ignore
    async def wrapper(*args, **kwargs):
        await handler(*args, **kwargs)


async def loop() -> None:
    while True:
        try:
            if message_queue or message_dequeue:
                coros1 = [
                    _emit(event, data, target_id)
                    for target_id, event, data in message_queue
                ]
                coros2 = [
                    _on(event, handler, namespace)
                    for event, handler, namespace in message_dequeue
                ]

                message_queue.clear()
                message_dequeue.clear()
                for coro in coros1:
                    try:
                        await coro
                    except Exception as e:
                        print(f"Emit error: {e}")
                for coro in coros2:
                    try:
                        await coro
                    except Exception as e:
                        print(f"Handler registration error: {e}")

        except Exception as e:
            print(f"Loop error: {e}")
        finally:
            await asyncio.sleep(0.0001)
