#!/usr/bin/env bash

name="sun-sets-gtk-theme"

systemctl --user disable --now "$name.service"

CONFIG_DIR=${XDG_CONFIG_DIRS:-"$HOME/.config"}
rm -f "$CONFIG_DIR/systemd/user/$name.service"
rm -f "$HOME/.local/bin/$name"

systemctl --user daemon-reload
