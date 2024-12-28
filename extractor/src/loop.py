import asyncio


async def async_loop():
    while True:
        print("Hello, world!")
        await asyncio.sleep(5)
