name = "Album Art"
description = "Syncs lights with the dominant color of the album art of the currently playing track."


async def run(options, devices):
    import lightkit

    async def cb(data):
        if "image" in data:
            r, g, b = lightkit.get_dominant_color(data["image"])
            h, s, l = lightkit.rgb_to_hsl(r, g, b)
            await lightkit.batch(devices, lambda d: d.set_color(h, s, l))

    await lightkit.on_media_player_track_changed(cb)
