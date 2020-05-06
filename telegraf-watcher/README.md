# Telegraf Watcher

Small service to restart telegraf on modification of config file.

## Installation

Installation requires `wget` and assumes both `telegraf` and [`Rust`](https://www.rust-lang.org/tools/install) are already installed on your machine.

```
wget https://github.com/mikemahoney218/pi-admin/raw/master/telegraf-watcher/target/debian/telegraf-watcher_0.1.3_armhf.deb && \
    sudo dpkg -i telegraf-watcher_0.1.2_armhf.deb && \
    sudo wget https://raw.githubusercontent.com/mikemahoney218/pi-admin/master/telegraf-watcher/telegraf-watcher.service -O /lib/systemd/system/telegraf-watcher.service && \
    sudo systemctl daemon-reload && \
    sudo systemctl enable --now telegraf-watcher
```
