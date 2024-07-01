import asyncio
import json
import os
import struct
import subprocess
import tempfile
from typing import Self, TypedDict
import audioplayer
import random
import dbus_next
from gi.repository import GLib

SOUND_DIR = f"{GLib.get_user_config_dir()}/cute_lights/effects/sounds"


class SoundPlayer:
    def __init__(self, sound: str, loop=False) -> None:
        if not os.path.exists(f"{SOUND_DIR}/{sound}"):
            raise FileNotFoundError(f"Sound file {sound} not found")
        self.player = audioplayer.AudioPlayer(f"{SOUND_DIR}/{sound}")
        self.loop = loop

    def __enter__(self) -> Self:
        self.start()
        return self

    def __exit__(self, exc_type, exc_value, traceback) -> None:
        self.stop()

    def stop(self) -> None:
        self.player.stop()

    def start(self) -> None:
        self.player.play(loop=self.loop, block=False)


class TrackMetadata(TypedDict):
    title: str
    album: str
    image: str

    @staticmethod
    def from_mpris(metadata: dict) -> Self:
        return {
            "title": (
                metadata["xesam:title"].value if "xesam:title" in metadata else None
            ),
            "album": (
                metadata["xesam:album"].value if "xesam:album" in metadata else None
            ),
            "image": (
                metadata["mpris:artUrl"].value.replace("file://", "")
                if "mpris:artUrl" in metadata
                else None
            ),
        }


async def on_media_player_track_changed(cb):
    script_dir = os.path.dirname(os.path.realpath(__file__))
    with open(f"{script_dir}/mpris.xml", "r") as f:
        mpris_introspection = f.read()

    bus = await dbus_next.aio.MessageBus(bus_type=dbus_next.BusType.SESSION).connect()

    bus_names_reply = await bus.call(
        dbus_next.Message(
            destination="org.freedesktop.DBus",
            path="/org/freedesktop/DBus",
            interface="org.freedesktop.DBus",
            member="ListNames",
        )
    )

    if bus_names_reply.message_type == dbus_next.MessageType.ERROR:
        raise Exception(bus_names_reply.body[0])

    dbus_name = None

    for name in bus_names_reply.body[0]:
        if name.startswith("org.mpris.MediaPlayer2."):
            dbus_name = name
            break

    player = bus.get_proxy_object(
        dbus_name, "/org/mpris/MediaPlayer2", mpris_introspection
    ).get_interface("org.freedesktop.DBus.Properties")

    meta = await player.call_get("org.mpris.MediaPlayer2.Player", "Metadata")
    await cb(TrackMetadata.from_mpris(meta.value))

    async def on_properties_changed(interface, changed, invalidated):
        if "Metadata" in changed:
            await cb(TrackMetadata.from_mpris(changed["Metadata"].value))

    player.on_properties_changed(on_properties_changed)

    await asyncio.get_event_loop().create_future()



async def visualizer(cb):
    BARS_NUMBER = 1
    RAW_TARGET = "/dev/stdout"

    config = """
    [general]
    bars = 1
    [output]
    method = raw
    raw_target = /dev/stdout
    bit_format = 16bit
    """
    byte_type, byte_size, byte_norm = ("H", 2, 65535)
    with tempfile.NamedTemporaryFile() as config_file:
        config_file.write(config.encode())
        config_file.flush()

        process = subprocess.Popen(["cava", "-p", config_file.name], stdout=subprocess.PIPE)
        chunk = byte_size * BARS_NUMBER
        fmt = byte_type * BARS_NUMBER

        if RAW_TARGET != "/dev/stdout":
            if not os.path.exists(RAW_TARGET):
                os.mkfifo(RAW_TARGET)
            source = open(RAW_TARGET, "rb")
        else:
            source = process.stdout

        while True:
            data = source.read(chunk)
            if len(data) < chunk:
                break
            
            sample = [i / byte_norm for i in struct.unpack(fmt, data)]
            await cb(max(sample))
