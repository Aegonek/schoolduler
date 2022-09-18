use std::error::Error;
use std::fmt::Arguments;
use std::fs::File;
use std::io::{self, Write};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};
use crate::utils;
use time::OffsetDateTime;

pub struct Logger {
    sender: Sender<String>,
    handle: JoinHandle<()>
}

impl Logger {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (tx, rx) : (Sender<String>, Receiver<_>) = mpsc::channel();

        let start_time = OffsetDateTime::now_local()?;
        let path = utils::time::timestamp_path("output/log.txt", start_time);
        let file = utils::fs::create_file_all(path)?;

        let handle = thread::spawn(move || {
            let receiver = rx;
            let file = file;
            loop {
                let msg = rx.recv().unwrap();
                println!("{msg}");
                writeln!(file, "{msg}");
            }
        });
        
        todo!();
        // Ok(logger)
    }

    pub fn log(&mut self, args: Arguments) -> Result<(), io::Error> {
        args.to_string();
        println!("{args}");
        writeln!(self.log, "{args}")
    }
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $($x:tt)*) => {
            $logger.log(format_args!($($x)*))
    };
}

pub use log;
