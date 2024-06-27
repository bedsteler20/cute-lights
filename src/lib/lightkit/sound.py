import os
import audioplayer
import random
from gi.repository import GLib

SOUND_DIR = f"{GLib.get_user_config_dir()}/cute_lights/effects/sounds"

_handles: dict[int, audioplayer.AudioPlayer] = {}

def play_sound(sound: str, loop=False) -> int:
    if not os.path.exists(f"{SOUND_DIR}/{sound}"):
        raise FileNotFoundError(f"Sound file {sound} not found")
    handle = random.randint(0, 2**32)
    player = audioplayer.AudioPlayer(f"{SOUND_DIR}/{sound}")
    player.play(loop=loop, block=False)
    _handles[handle] = player

    return handle

def stop_sound(handle: int) -> None:
    if handle in _handles:
        _handles[handle].stop()
        del _handles[handle]
    else:
        raise ValueError(f"Invalid sound handle {handle}")