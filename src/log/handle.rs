use crate::utils;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use time::OffsetDateTime;

use super::comm::{LoggerError, Message};
use super::logger::LoggerImpl;
use super::HashCode;

pub struct LogHandle {
    // Every handle has Some(sender), this is an Option only so I can explicitly drop sender in Drop
    sender: Option<Sender<Message>>,
    // Handle is Some(sender) for `master` handle, and None for `child` handles
    handle: Option<JoinHandle<()>>,
    is_poisoned: Arc<AtomicBool>,
    start_time: OffsetDateTime,
}

impl LogHandle {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (tx, rx): (Sender<Message>, Receiver<_>) = mpsc::channel();

        let start_time = OffsetDateTime::now_local()?;
        #[cfg(not(test))]
        let path = utils::time::timestamp_path("output/log.txt", start_time);
        #[cfg(test)]
        let path = "output/log.txt";
        let file = utils::fs::create_file_all(path)?;
        let is_poisoned = Arc::new(AtomicBool::new(false));

        let handle = {
            let is_poisoned = is_poisoned.clone();
            thread::spawn(move || {
                let mut logger = LoggerImpl::new(file, rx);
                loop {
                    match logger.receive() {
                        Ok(_) => (),
                        Err(err) => match err {
                            LoggerError::SenderDisconnected => {
                                logger
                                    .log("Disposing the logger...".to_string())
                                    .expect("Unexpected error: filesystem error!");
                                return;
                            }
                            err @ LoggerError::FileSystemError(_) => {
                                is_poisoned.store(true, Ordering::SeqCst);
                                eprintln!("Unexpected error: fatal logger error! {err}");
                                panic!();
                            }
                        },
                    }
                }
            })
        };
        let logger = LogHandle {
            sender: Some(tx),
            handle: Some(handle),
            is_poisoned,
            start_time,
        };

        Ok(logger)
    }

    pub fn log<T: ToOwned<Owned = String>>(&self, msg: T) {
        send(self, Message::Log(msg.to_owned()));
        if self.is_poisoned.load(Ordering::Relaxed) {
            panic!()
        }
    }

    pub fn store<T: ToOwned<Owned = String>>(&self, key: HashCode, msg: T) {
        send(self, Message::Store(key, msg.to_owned()))
    }

    pub fn commit(&self, key: HashCode) {
        send(self, Message::Commit(key));
        if self.is_poisoned.load(Ordering::Relaxed) {
            panic!()
        }
    }

    pub fn flush(&self) {
        send(self, Message::Flush)
    }

    pub fn start_time(&self) -> OffsetDateTime {
        self.start_time
    }
}

impl Drop for LogHandle {
    fn drop(&mut self) {
        drop(self.sender.take().unwrap());
        self.handle.take().map(|hnd| hnd.join().unwrap());
    }
}

impl Clone for LogHandle {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            handle: None,
            is_poisoned: self.is_poisoned.clone(),
            start_time: self.start_time.clone(),
        }
    }
}

fn send(logger: &LogHandle, msg: Message) {
    logger.sender.as_ref().unwrap().send(msg).unwrap();
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $($x:tt)*) => {
            $logger.log(format!($($x)*))
    };
}

#[macro_export]
macro_rules! store {
    ($logger:expr, $key:expr, $($x:tt)*) => {
            $logger.store($key, format!($($x)*))
    };
}

pub use {log, store};
