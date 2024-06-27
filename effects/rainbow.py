name = "Rainbow"
description = "A simple rainbow effect that cycles through the color wheel."

async def run(options, devices):
    import lightkit
    color = 0
    while True:
        await lightkit.batch(devices, lambda light: light.set_color(color, 100, 50, 1))
        await lightkit.sleep(1)
        color = (color + 4) % 360
