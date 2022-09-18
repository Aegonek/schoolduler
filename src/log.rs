use crate::utils;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvError, Sender};
use std::sync::Arc;
use std::thread;
use time::OffsetDateTime;

pub use LogHandle as Logger;

type HashCode = u64;

struct LoggerImpl {
    file: File,
    receiver: Receiver<Message>,
    store: HashMap<HashCode, Vec<String>>,
}

#[derive(Debug)]
enum LoggerError {
    SenderDisconnected,
    FileSystemError(io::Error),
}

impl Display for LoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for LoggerError {}

impl From<RecvError> for LoggerError {
    fn from(_: RecvError) -> Self {
        LoggerError::SenderDisconnected
    }
}

impl From<io::Error> for LoggerError {
    fn from(err: io::Error) -> Self {
        LoggerError::FileSystemError(err)
    }
}

impl LoggerImpl {
    fn new(file: File, receiver: Receiver<Message>) -> Self {
        LoggerImpl {
            file,
            receiver,
            store: HashMap::new(),
        }
    }

    fn receive(&mut self) -> Result<(), LoggerError> {
        use Message::*;

        match self.receiver.recv()? {
            Log(msg) => self.log(msg)?,
            Store(key, msg) => self.store(key, msg),
            Commit(key) => self.commit(key)?,
            Flush => self.flush(),
        }
        Ok(())
    }

    fn log(&mut self, msg: String) -> Result<(), io::Error> {
        println!("{msg}");
        writeln!(self.file, "{msg}")?;
        Ok(())
    }

    fn store(&mut self, key: HashCode, msg: String) {
        self.store.entry(key).or_insert(Vec::new()).push(msg);
    }

    fn commit(&mut self, key: HashCode) -> Result<(), io::Error> {
        if let Some(msgs) = self.store.remove(&key) {
            for msg in msgs {
                self.log(msg)?;
            }
        }
        Ok(())
    }

    fn flush(&mut self) {
        self.store = HashMap::new();
    }
}

enum Message {
    Log(String),
    // Why?
    // We want to log example data from iterations, but we don't want to do it for every chromosome to not bloat logs
    // So we want to delay writing to files until we know which chromosome won, so we can log only his data.

    // Request that logger stores entry under given HashCode
    Store(HashCode, String),
    // Commit entries under given HashCode to file / stdout
    Commit(HashCode),
    // Flush entries stored internally in Logger
    Flush,
}

pub struct LogHandle {
    sender: Sender<Message>,
    is_poisoned: Arc<AtomicBool>,
    start_time: OffsetDateTime,
}

impl LogHandle {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (tx, rx): (Sender<Message>, Receiver<_>) = mpsc::channel();

        let start_time = OffsetDateTime::now_local()?;
        let path = utils::time::timestamp_path("output/log.txt", start_time);
        let file = utils::fs::create_file_all(path)?;
        let is_poisoned = Arc::new(AtomicBool::new(false));

        let logger = LogHandle {
            sender: tx,
            is_poisoned: is_poisoned.clone(),
            start_time,
        };

        let _ = thread::spawn(move || {
            let mut logger = LoggerImpl::new(file, rx);
            let is_poisoned = is_poisoned;
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
                        // This error won't propagate to main thread, which is a bit yikes.
                        err @ LoggerError::FileSystemError(_) => {
                            is_poisoned.store(true, Ordering::SeqCst);
                            eprintln!("Unexpected error: fatal logger error! {err}");
                            panic!();
                        }
                    },
                }
            }
        });

        Ok(logger)
    }

    pub fn log(&mut self, msg: String) {
        self.sender.send(Message::Log(msg)).unwrap();
        if self.is_poisoned.load(Ordering::Relaxed) {
            panic!()
        }
    }

    pub fn store(&mut self, key: HashCode, msg: String) {
        self.sender.send(Message::Store(key, msg)).unwrap()
    }

    pub fn commit(&mut self, key: HashCode) {
        self.sender.send(Message::Commit(key)).unwrap();
        if self.is_poisoned.load(Ordering::Relaxed) {
            panic!()
        }
    }

    pub fn flush(&mut self) {
        self.sender.send(Message::Flush).unwrap()
    }

    pub fn start_time(&self) -> OffsetDateTime {
        self.start_time
    }
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

// TODO: test
