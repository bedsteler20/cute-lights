name = "Pop Out"
description = "Lights will randomly turn on and off."

async def run(options, devices):
    import lightkit
    import random
    await lightkit.batch(devices, lambda d: d.off())
    while True:
        light = random.choice(devices)
        color = random.randint(0, 360)
        saturation = random.randint(50, 100)
        brightness = 100
        await light.set_color(color, saturation, brightness)
        await light.on()
        while brightness > 0:
            await light.set_color(color, saturation, brightness)
            await lightkit.sleep(1)
            brightness -= 20
        await light.off()
