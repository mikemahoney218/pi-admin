extern crate inotify;

use inotify::{Inotify, WatchMask};
use std::process::Command;

fn main() {
    let mut inotify = Inotify::init()
        .expect("Error while initializing inotify instance");

    

    loop {

        inotify
        .add_watch(
            "/etc/telegraf/telegraf.conf",
            WatchMask::MODIFY,
        )
        .expect("Failed to add file watch");

        let mut buffer = [0; 1024];
        let _events = inotify.read_events_blocking(&mut buffer)
        .expect("Error while reading events");

        let _output = Command::new("sh")
                                 .arg("-c")
                                 .arg("systemctl restart telegraf")
                                 .status()
                                 .expect("Failed to execute command");

    }

}
