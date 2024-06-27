# window.py
#
# Copyright 2024 Cameron Dehning
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#
# SPDX-License-Identifier: GPL-3.0-or-later

from gi.repository import Adw
from gi.repository import Gtk, GObject, GLib
from .effect_loader import EffectLoader

effect_loader = EffectLoader()


@Gtk.Template(resource_path="/dev/bedsteler20/CuteLights/window.ui")
class CuteLightsWindow(Adw.PreferencesWindow):
    __gtype_name__ = "CuteLightsWindow"

    group: Adw.PreferencesGroup = Gtk.Template.Child()

    active_effect: str = GObject.Property(type=str, default="")

    _rows: list[Adw.SwitchRow] = []

    def __init__(self, **kwargs):
        super().__init__(**kwargs)       
        x = effect_loader.get_active_effect()
        if x:
            self.active_effect = x
        for effect in effect_loader.effects:
            row = self.build_effect_row(effect)
            self.group.add(row)
            self._rows.append(row)


    skip: bool = False

    def build_effect_row(self, effect):
        row = Adw.SwitchRow()
        row.set_title(effect.name)
        row.set_subtitle(effect.description)

        if self.active_effect == effect.effect_id:
            row.set_active(True)
        
        def unskip(*args):
            self.skip = False

        def on_switch_changed(switch, state):
            if self.skip:
                return
            if row.get_active():
                effect_loader.stop_effect()
                effect_loader.start_effect(effect.effect_id)
                self.skip = True
                self.active_effect = effect.effect_id
                GLib.idle_add(unskip)
            else:
                effect_loader.stop_effect()
                self.active_effect = ""

        def on_active_effect_changed(*args):
            if self.active_effect != effect.effect_id:
                row.set_active(False)

        row.connect("notify::active", on_switch_changed)
        self.connect("notify::active-effect", on_active_effect_changed)

        return row
