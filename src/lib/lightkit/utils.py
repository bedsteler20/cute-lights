import asyncio

from lightkit.core import Light


async def sleep(seconds: int):
    await asyncio.sleep(seconds)


async def batch(devices: list[Light], func):
    await asyncio.gather(*[func(d) for d in devices])
