extern crate inotify;
extern crate log;
extern crate simple_logger;


use inotify::{Inotify, WatchMask};
use std::process::Command;
use log::{info, error};

fn main() {
    simple_logger::init().unwrap();

    info!("Telegraf watcher service initialized.");

    let mut inotify = Inotify::init()
                      .unwrap_or_else(|error| {
                          error!("{:?}", error);
                          panic!("{:?}", error);
                        });

    loop {

        inotify
        .add_watch(
            "/etc/telegraf/telegraf.conf",
            WatchMask::MODIFY,
        )
        .unwrap_or_else(|error| {
            error!("{:?}", error);
            panic!("{:?}", error);
        });

        let mut buffer = [0; 1024];
        let _events = inotify.read_events_blocking(&mut buffer)
        .unwrap_or_else(|error| {
            error!("{:?}", error);
            panic!("{:?}", error);
        });

        let _output = Command::new("sh")
                                 .arg("-c")
                                 .arg("systemctl restart telegraf")
                                 .status()
                                 .unwrap_or_else(|error| {
                                    error!("{:?}", error);
                                    panic!("{:?}", error);
                                  });

    }

}
