#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, StdoutLock, Write};
use std::sync::mpsc::Receiver;
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
            Store(key, (severity, msg)) => self.store(key, severity, msg)?,
            Commit(key) => self.commit(key)?,
            Flush => self.flush(),
        }
        Ok(())
    }

    pub fn info<T: ToString>(&mut self, msg: T) -> Result<(), io::Error> {
        self.log(Severity::Info, msg)
    }

    pub fn warning<T: ToString>(&mut self, msg: T) -> Result<(), io::Error> {
        self.log(Severity::Warning, msg)
    }

    pub fn error<T: ToString>(&mut self, msg: T) -> Result<(), io::Error> {
        self.log(Severity::Error, msg)
    }

    pub fn log<T: ToString>(&mut self, severity: Severity, msg: T) -> Result<(), io::Error> {
        let msg = format_msg(severity, msg.to_string())?;
        self.write(msg)
    }

    pub fn store<T: ToString>(
        &mut self,
        key: HashCode,
        severity: Severity,
        msg: T,
    ) -> Result<(), io::Error> {
        let msg = format_msg(severity, msg.to_string())?;
        self.store
            .entry(key)
            .or_insert(Vec::new())
            .push(msg.to_string());
        Ok(())
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

    fn write<T: Display>(&mut self, msg: T) -> Result<(), io::Error> {
        println!("{msg}");
        writeln!(self.file, "{msg}")?;
        Ok(())
    }
}

fn format_msg(severity: Severity, msg: String) -> Result<String, io::Error> {
    let now = OffsetDateTime::now_local()
        .map_err(|_| io::Error::other("Couldn't retrieve timezone information"))?;
    let fmt = format!("{} [{severity}]: {msg}", now.time());
    Ok(fmt)
}
