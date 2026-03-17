# Nexis Python SDK

Python client library for the Nexis API.

## Installation

```bash
pip install nexis-sdk
```

## Usage

```python
import asyncio
from nexis import NexisClient

async def main():
    async with NexisClient("https://api.nexis.io") as client:
        result = await client.login("user@example.com", "password")
        room = await client.create_room(name="General")
        await client.join_room(room.id)
        msg = await client.send_message(room.id, "Hello!")
        print(msg.content)

asyncio.run(main())
```

## WebSocket

```python
from nexis.websocket import WebSocketConnection

ws = WebSocketConnection("ws://api.nexis.io/ws?room_id=xxx", token)
await ws.connect()
ws.on_message(lambda m: print(m))
asyncio.create_task(ws.listen())
```
