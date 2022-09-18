use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use super::comm::{Message, LoggerError};
use super::HashCode;

pub struct LoggerImpl {
    file: File,
    receiver: Receiver<Message>,
    store: HashMap<HashCode, Vec<String>>,
}

impl LoggerImpl {
    pub fn new(file: File, receiver: Receiver<Message>) -> Self {
        LoggerImpl {
            file,
            receiver,
            store: HashMap::new(),
        }
    }

    pub fn receive(&mut self) -> Result<(), LoggerError> {
        use Message::*;

        match self.receiver.recv()? {
            Log(msg) => self.log(msg)?,
            Store(key, msg) => self.store(key, msg),
            Commit(key) => self.commit(key)?,
            Flush => self.flush(),
        }
        Ok(())
    }

    pub fn log(&mut self, msg: String) -> Result<(), io::Error> {
        println!("{msg}");
        writeln!(self.file, "{msg}")?;
        Ok(())
    }

    pub fn store(&mut self, key: HashCode, msg: String) {
        self.store.entry(key).or_insert(Vec::new()).push(msg);
    }

    pub fn commit(&mut self, key: HashCode) -> Result<(), io::Error> {
        if let Some(msgs) = self.store.remove(&key) {
            for msg in msgs {
                self.log(msg)?;
            }
        }
        Ok(())
    }

    pub fn flush(&mut self) {
        self.store = HashMap::new();
    }
}