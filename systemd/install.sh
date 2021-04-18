#!/usr/bin/env bash
set -e

nice cargo install --path ..

mkdir -p ~/.config/systemd/user/
cp -uv ./*.service ~/.config/systemd/user/

systemctl --user daemon-reload
systemctl --user restart sun-sets-gtk-theme.service
systemctl --user enable sun-sets-gtk-theme.service
