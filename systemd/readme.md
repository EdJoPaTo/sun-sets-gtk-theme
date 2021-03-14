# systemd service

The install scripts assumes this repo to be cloned locally and having rust, cargo and so on installed.

Adapt the service file command to have the themes you like.
Then run the install script.

Check the status of the service with `systemctl --user status sun-sets-gtk-theme.service -n 100` where `-n <number>` is the amount of log lines you wanna see.
