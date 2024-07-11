name = "Visulizer"
description = "Visualizer for audio"


async def run(options, devices):
    import lightkit

    steps = 10

    presission = 0.01
    class callback:
        prev_val = 0
        async def cb(self, val):
            h, s, l = (int((360*presission) * val), 100, 100)
            if h  == self.prev_val:
                return
            self.prev_val = h
            print(h)
            await lightkit.batch(devices, lambda d: d.set_color(int(h / presission), s, l))

    await lightkit.visualizer(callback().cb)