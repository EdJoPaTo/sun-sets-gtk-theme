# systemd service

The installation script assumes this repo to be cloned locally and having rust, cargo and so on installed.

Adapt the service file command to have the themes you like.
Then run the installation script from the main folder: `./systemd/user/install.sh`.

Check the status of the service with `systemctl --user status sun-sets-gtk-theme.service -n 100` where `-n <number>` is the amount of log lines you wanna see.
