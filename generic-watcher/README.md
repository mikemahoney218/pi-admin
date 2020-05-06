# Generic Configuration Watcher

This app is an expansion of the [Telegraf watcher](../telegraf-watcher) 
service, which will watch any number of service configuration files (specified
at `/etc/generic-watcher/generic-watcher.toml` in the format 
`service = configuration file`) and restart the service following any writes.

## Installation

```
wget https://github.com/mikemahoney218/pi-admin/raw/master/generic-watcher/target/debian/generic-watcher_0.1.0_armhf.deb && \
    sudo dpkg -i generic-watcher_0.1.0_armhf.deb && \
    sudo wget https://raw.githubusercontent.com/mikemahoney218/pi-admin/master/generic-watcher/generic-watcher.service -O /lib/systemd/system/generic-watcher.service && \
    sudo systemctl daemon-reload && \
    sudo systemctl enable --now generic-watcher
```
