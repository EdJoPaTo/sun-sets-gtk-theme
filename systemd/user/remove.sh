#!/usr/bin/env bash

systemctl --user disable --now "sun-sets-gtk-theme.service"

CONFIG_DIR=${XDG_CONFIG_DIRS:-"$HOME/.config"}
rm -f "$CONFIG_DIR/systemd/user/sun-sets-gtk-theme.service"
rm -f "$HOME/.local/bin/sun-sets-gtk-theme"

systemctl --user daemon-reload
