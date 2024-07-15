import asyncio
import time
import cute_light

lights = cute_light.discover_lights()
frame = cute_light.Frame()

state = True
while True:
    frame.set_on_all(lights, state)
    state = not state
    frame.run()
    frame.clear()
    time.sleep(0.5)