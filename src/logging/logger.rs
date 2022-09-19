#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, StdoutLock, Write};
use std::sync::mpsc::Receiver;
use time::macros::format_description;
use time::OffsetDateTime;

use super::comm::{LoggerError, Message, Severity};
use super::HashCode;

pub struct LoggerImpl {
    file: File,
    lock: StdoutLock<'static>,
    receiver: Receiver<Message>,
    store: HashMap<HashCode, Vec<String>>,
}

impl LoggerImpl {
    pub fn new(file: File, receiver: Receiver<Message>) -> Self {
        let lock = io::stdout().lock();
        LoggerImpl {
            file,
            lock,
            receiver,
            store: HashMap::new(),
        }
    }

    pub fn receive(&mut self) -> Result<(), LoggerError> {
        use Message::*;

        match self.receiver.recv()? {
            Log(severity, msg) => self.log(severity, msg)?,
            Store(key, (severity, msg)) => self.store(key, severity, msg),
            Commit(key) => self.commit(key)?,
            Flush => self.flush(),
        }
        Ok(())
    }

    pub fn info(&mut self, msg: String) -> Result<(), io::Error> {
        self.log(Severity::Info, msg)
    }

    pub fn warning(&mut self, msg: String) -> Result<(), io::Error> {
        self.log(Severity::Warning, msg)
    }

    pub fn error(&mut self, msg: String) -> Result<(), io::Error> {
        self.log(Severity::Error, msg)
    }

    fn log(&mut self, severity: Severity, msg: String) -> Result<(), io::Error> {
        let msg = format_msg(severity, msg);
        self.write(msg)
    }

    pub fn store(&mut self, key: HashCode, severity: Severity, msg: String) {
        let msg = format_msg(severity, msg);
        self.store
            .entry(key)
            .or_insert(Vec::new())
            .push(msg.to_string());
    }

    pub fn commit(&mut self, key: HashCode) -> Result<(), io::Error> {
        if let Some(msgs) = self.store.remove(&key) {
            for msg in msgs {
                self.write(msg)?;
            }
        }
        Ok(())
    }

    pub fn flush(&mut self) {
        self.store = HashMap::new();
    }

    fn write(&mut self, msg: String) -> Result<(), io::Error> {
        println!("{msg}");
        writeln!(self.file, "{msg}")?;
        Ok(())
    }
}

fn format_msg(severity: Severity, msg: String) -> String {
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let fmt = format!(
        "{} [{severity}]: {msg}",
        now.time()
            .format(format_description!("[hour]:[minute]:[second]"))
            .unwrap()
    );
    fmt
}
