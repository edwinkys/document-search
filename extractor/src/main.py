import os
import asyncio
from multiprocessing import Process
from rich.console import Console
from typer import Typer
from uvicorn import run
from src.core.loop import async_loop

cli = Typer(help="Interface to manage the extractor worker.")
console = Console()


def start_loop():
    try:
        asyncio.run(async_loop())
    except KeyboardInterrupt:
        console.print(" | Keyboard interruption detected. Stopping...")
        console.log("INFO: The extraction loop has been stopped", style="green")
        pass


def start_api():
    port = int(os.getenv("DL_EXTRACTOR_PORT", 2510))
    run("src.core.api:api", host="0.0.0.0", port=port, reload=False)


@cli.command()
def start():
    """Start the extractor worker."""

    processes = []

    # Start the API server in a separate process.
    api_process = Process(target=start_api)
    processes.append(api_process)
    api_process.start()

    # Start the async loop in a separate process.
    async_process = Process(target=start_loop)
    processes.append(async_process)
    async_process.start()

    for process in processes:
        process.join()


if __name__ == "__main__":
    cli()
