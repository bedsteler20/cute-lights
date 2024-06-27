name = "Pink"
description = "Sets all lights to pink."


async def run(options, devices):
    import lightkit
    await lightkit.batch(devices, lambda d: d.set_color(300, 100, 70))
