import os
import requests
import uuid
import asyncio
from rich.console import Console
from ..utils.coordinator import Coordinator

# Time to sleep between iterations in seconds.
SLEEP = 5

console = Console()


async def async_loop():
    host = os.getenv("DL_EXTRACTOR_HOST", external_ip_address())
    port = int(os.getenv("DL_EXTRACTOR_PORT", 2510))
    worker = {
        "id": str(uuid.uuid4()),
        "address": f"{host}:{port}",
    }

    coordinator_addr = os.getenv("DL_COORDINATOR_ADDR", "0.0.0.0:2500")
    coordinator: Coordinator
    should_register = True

    while True:
        try:
            coordinator = Coordinator(address=coordinator_addr)
            coordinator.heartbeat()
        except Exception:
            message = "ERROR: Failed to connect to the coordinator server"
            console.log(message, style="red")
            should_register = True
            await asyncio.sleep(SLEEP)
            continue

        # When the worker starts or loses connection to the coordinator,
        # it must register itself again.
        if should_register:
            coordinator.register_worker(**worker)
            should_register = False

            message = "INFO: Registered the worker to the coordinator server"
            console.log(message, style="green")

        print("Hello, world!")
        await asyncio.sleep(SLEEP)


def external_ip_address() -> str:
    response = requests.get("https://api.ipify.org?format=json")
    return response.json()["ip"]
