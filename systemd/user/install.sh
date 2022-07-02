#!/usr/bin/env bash
set -e

name="sun-sets-gtk-theme"

dir=$(basename "$(pwd)")
if [ "$dir" == "systemd" ] || [ "$dir" == "user" ]; then
    echo "run from main directiory like this: ./systemd/user/install.sh"
    exit 1
fi

# Create config/bin folders
CONFIG_DIR=${XDG_CONFIG_DIRS:-"$HOME/.config"}
mkdir -p "$CONFIG_DIR/systemd/user/" "$HOME/.local/bin"

nice cargo build --release --locked

# systemd
cp -v "systemd/user/service" "$CONFIG_DIR/systemd/user/$name.service"
systemctl --user daemon-reload

# stop, replace and start new version
systemctl --user stop "$name.service"
cp -v "target/release/$name" "$HOME/.local/bin"

systemctl --user enable --now "$name.service"
