import os
import requests
import uuid
import asyncio
import aio_pika
import json
from aio_pika.abc import AbstractIncomingMessage
from rich.console import Console
from uuid import UUID
from ..utils.coordinator import Coordinator
from ..utils.extraction import Extraction
from ..utils.types import ExtractionTask

QUEUE_NAME = "tasks"
SLEEP = 5

console = Console()


async def async_loop():
    queue_url = os.getenv("DL_QUEUE_URL")
    if not queue_url:
        message = "ERROR: Please provide a valid RabbitMQ URL"
        console.log(message, style="red")
        return

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

        # Attempt to connect to the RabbitMQ server.
        # Putting this inside the loop allows the worker to recover from
        # connection failures without stopping the process.
        try:
            queue_connection = await aio_pika.connect_robust(queue_url)
        except Exception:
            error = "ERROR: Failed to connect to the RabbitMQ server"
            console.log(error, style="red")
            await asyncio.sleep(SLEEP)
            continue

        async with queue_connection:
            channel = await queue_connection.channel()
            # We use get queue method since the queue is guaranteed to exist.
            queue = await channel.get_queue(QUEUE_NAME)
            message = await queue.get(no_ack=False, fail=False)
            if message is not None:
                await process_message(coordinator, message)

        await asyncio.sleep(SLEEP)


def external_ip_address() -> str:
    response = requests.get("https://api.ipify.org?format=json")
    return response.json()["ip"]


async def process_message(
    coordinator: Coordinator,
    message: AbstractIncomingMessage,
):
    async with message.process(ignore_processed=True):
        # The message is a JSON string.
        # Check server/src/types.rs for the ExtractionTask struct.
        body = json.loads(message.body.decode())
        namespace = body["namespace"]
        document_key = body["document_key"]
        document_id = body["document_id"]

        console.log(f"INFO: Processing document {document_id}...")
        coordinator.update_document(namespace, document_id, "processing")

        task = ExtractionTask(namespace, document_key, UUID(document_id))
        path = task.download_document()

        extraction = Extraction(path)
        results = extraction.extract()
        console.log(f"INFO: Extracted {len(results)} chunks from the document")

        coordinator.create_chunk(namespace, document_id, results)
        task.cleanup()
