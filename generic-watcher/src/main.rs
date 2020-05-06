extern crate glob;
extern crate config;
extern crate bimap;
extern crate log;
extern crate simple_logger;
extern crate notify;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::process::Command;
use std::string::String;
use log::{info, warn, error};
use config::*;
use glob::glob;
use bimap::BiMap;

fn main() {
    simple_logger::init().unwrap();
    info!("Telegraf watcher service initialized.");

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(500)).unwrap();
    info!("Watcher initialized.");

    let mut settings = Config::default();

    settings
        .merge(glob("/etc/generic-watcher/generic-watcher.toml")
                   .unwrap()
                   .map(|path| File::from(path.unwrap()))
                   .collect::<Vec<_>>())
        .unwrap();

    let settings = settings.try_into::<BiMap<String, String>>().unwrap();

    for (service, file) in &settings {
        watcher.watch(&file, RecursiveMode::Recursive).unwrap();
        info!("Started watching {}, tied to service {}", file, service);
    } 

    loop {

        match rx.recv() {
            Ok(event) => {
                if let DebouncedEvent::Write(pathing) = event {
                    let key_val = pathing.to_str().unwrap().to_string();
                    let restart_me = settings.get_by_right(&key_val).unwrap();

                    info!("Restarting {}.", &restart_me);
                    let mut command_str: String = "systemctl restart ".to_owned();
                    command_str.push_str(restart_me);

                    let _output = Command::new("sh")
                                 .arg("-c")
                                 .arg(command_str)
                                 .status()
                                 .unwrap_or_else(|error| {
                                    error!("{:?}", error);
                                    panic!("{:?}", error);
                                  });
                }
            },
            Err(e) => warn!("Watch error: {:?}", e),
         }

    }

}