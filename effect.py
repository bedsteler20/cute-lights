name = 'Demo Effect'
description = 'This is a demo effect'
options = {
    'brightness': int,
}

def validate(options):
    if options['brightness'] < 0 or options['brightness'] > 100:
        return 'Brightness must be between 0 and 100'
    return None

async def run(options, devices):
    import light_kit
    import asyncio
    color = 0
    while True:
        light_kit.batch.set_color(devices, color, 100, options['brightness'])
        await asyncio.sleep(1)
        color = (color + 4) % 360
