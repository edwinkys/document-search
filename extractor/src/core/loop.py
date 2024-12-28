import os
import asyncio
from rich.console import Console
from ..utils.coordinator import Coordinator

# Time to sleep between iterations in seconds.
SLEEP = 5

console = Console()


async def async_loop():
    coordinator_addr = os.getenv("DL_COORDINATOR_ADDR", "0.0.0.0:2500")
    coordinator: Coordinator

    while True:
        try:
            coordinator = Coordinator(address=coordinator_addr)
            coordinator.heartbeat()
        except Exception:
            message = "ERROR: Failed to connect to the coordinator server"
            console.log(message, style="red")
            await asyncio.sleep(SLEEP)
            continue

        print("Hello, world!")
        await asyncio.sleep(SLEEP)
