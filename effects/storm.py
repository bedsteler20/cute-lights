name = "Storm"
description = "A effect that simulates a thunder storm."

async def run(options, devices):
    import lightkit
    import random

    await lightkit.batch(devices, lambda d: d.set_color(0, 0, 100))
    await lightkit.batch(devices, lambda d: d.off())
    with lightkit.SoundPlayer("rain.mp3"):
        while True:
            delay = random.randint(10, 20)
            print(f"Waiting {delay} seconds")
            await lightkit.sleep(delay)

            light = random.choice(devices)
            await light.on()
            await light.set_color(0, 0, 100)
            await lightkit.sleep(0.12)
            await light.off()
            with lightkit.SoundPlayer("thunder.mp3"):
                await lightkit.sleep(0.5)