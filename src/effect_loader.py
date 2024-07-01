#!/usr/bin/env python3
import subprocess
import importlib
import importlib.util
import json
import os
import sys
import asyncio

from gi.repository import GLib
import psutil


EFFECTS_DIR = f"{GLib.get_user_config_dir()}/cute_lights/effects"
STATE_DIR = f"{GLib.get_user_config_dir()}/cute_lights/state"

APP_CONF_FILE = f"{GLib.get_user_config_dir()}/cute_lights/settings.json"
ACTIVE_EFFECT_FILE = f"{STATE_DIR}/active_effect"
EFFECT_PID_FILE = f"{STATE_DIR}/effect_pid"
script_dir = os.path.dirname(os.path.realpath(__file__))


class Settings:
    govee_enabled: bool = False
    govee_ips: list[str] = []

    kasa_enabled: bool = False
    kasa_ips: list[str] = []

    hue_enabled: bool = False
    hue_bridge_ip: str = ""
    smartthings_enabled: bool = False
    smartthings_api_token: str = ""

    def __init__(self) -> None:
        if not os.path.exists(APP_CONF_FILE):
            self.save()
        else:
            self.load()

    def save(self) -> None:
        with open(APP_CONF_FILE, "w") as f:
            f.write(self.to_json())

    def load(self) -> None:
        with open(APP_CONF_FILE, "r") as f:
            self.from_json(f.read())

    def to_json(self) -> str:
        return json.dumps(self.__dict__)

    def from_json(self, json_str: str) -> None:
        data = json.loads(json_str)

        if "govee_enabled" in data:
            self.govee_enabled = data["govee_enabled"]

        if "govee_ips" in data:
            self.govee_ips = data["govee_ips"]

        if "kasa_enabled" in data:
            self.kasa_enabled = data["kasa_enabled"]

        if "kasa_ips" in data:
            self.kasa_ips = data["kasa_ips"]

        if "hue_enabled" in data:
            self.hue_enabled = data["hue_enabled"]

        if "hue_bridge_ip" in data:
            self.hue_bridge_ip = data["hue_bridge_ip"]

        if "smartthings_api_token" in data:
            self.smartthings_api_token = data["smartthings_api_token"]

        if "smartthings_enabled" in data:
            self.smartthings_enabled = data["smartthings_enabled"]


settings = Settings()


class EffectLoader:
    def __init__(self):
        self.effects = []

        if not os.path.exists(EFFECTS_DIR):
            os.makedirs(EFFECTS_DIR)

        if not os.path.exists(STATE_DIR):
            os.makedirs(STATE_DIR)

        self.refresh()

    def refresh(self):
        self.effects = []
        for file in os.listdir(EFFECTS_DIR):
            if not file.endswith(".py"):
                continue
            mod_name = file.replace(".py", "")
            spec = importlib.util.spec_from_file_location(
                mod_name, f"{EFFECTS_DIR}/{file}"
            )
            mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(mod)
            mod.path = f"{EFFECTS_DIR}/{file}"
            mod.effect_id = mod_name
            self.effects.append(mod)
            print(f"Loaded effect: {mod.name}")

    def start_effect(self, effect_id):
        porc = subprocess.Popen(
            ["python3", __file__, effect_id], start_new_session=True
        )
        with open(EFFECT_PID_FILE, "w") as f:
            f.write(str(porc.pid))

        with open(ACTIVE_EFFECT_FILE, "w") as f:
            f.write(effect_id)

    def stop_effect(self):
        if not os.path.exists(EFFECT_PID_FILE):
            return
        with open(EFFECT_PID_FILE, "r") as f:
            pid = int(f.read())
            if self._is_process_running(pid):
                os.system(f"kill -9 {pid}")

        os.remove(EFFECT_PID_FILE)
        os.remove(ACTIVE_EFFECT_FILE)

    def _is_process_running(self, pid):
        for proc in psutil.process_iter():
            if proc.pid == pid:
                return True

    def get_active_effect(self):
        if not os.path.exists(ACTIVE_EFFECT_FILE):
            return None

        with open(ACTIVE_EFFECT_FILE, "r") as f:
            return f.read()


if __name__ == "__main__":

    async def main():
        sys.path.insert(1, f"{script_dir}/lib")

        from lightkit.core import LightKit
        from lightkit.govee import GoveeDiscoverer
        from lightkit.kasa import KasaDiscoverer
        from lightkit.hue import HueDiscoverer
        from lightkit.smartthings import SmartThingsDiscover

        kit = LightKit()

        if settings.govee_enabled:
            kit.add_discoverer(GoveeDiscoverer(settings.govee_ips))

        if settings.kasa_enabled:
            kit.add_discoverer(KasaDiscoverer(settings.kasa_ips))

        if settings.hue_enabled:
            kit.add_discoverer(HueDiscoverer(settings.hue_bridge_ip))

        if settings.smartthings_enabled:
            kit.add_discoverer(SmartThingsDiscover(settings.smartthings_api_token))

        mod_name = sys.argv[1]
        spec = importlib.util.spec_from_file_location(
            mod_name, f"{EFFECTS_DIR}/{mod_name}.py"
        )
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)

        options = {}
        if len(sys.argv) > 2:
            options = json.loads(sys.argv[2])

        devices = await kit.discover()

        await mod.run(options, devices)

    asyncio.run(main())
