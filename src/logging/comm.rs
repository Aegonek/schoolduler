use std::error::Error;
use std::fmt::Display;
use std::io;
use std::sync::mpsc::RecvError;

use super::HashCode;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    Info,
    Warning,
    Error
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_uppercase())
    }
}

#[derive(Debug)]
pub enum LoggerError {
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

pub enum Message {
    Log(Severity, String),
    // Why?
    // We want to log example data from iterations, but we don't want to do it for every chromosome to not bloat logs
    // So we want to delay writing to files until we know which chromosome won, so we can log only his data.

    // Request that logger stores entry under given HashCode
    Store(HashCode, (Severity, String)),
    // Commit entries under given HashCode to file / stdout
    Commit(HashCode),
    // Flush entries stored internally in Logger
    Flush,
}