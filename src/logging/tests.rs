use crate::logging::comm::Severity;

use super::{info, store, Logger};
use std::cmp::Ordering;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

// No running tests in this module in parallel.
static LOCK: Mutex<()> = Mutex::new(());

#[test]
fn log_succeeds() -> Result<(), Box<dyn Error>> {
    let _lock = LOCK.lock()?;
    let logger = Logger::new()?;
    info!(logger, "Foobar.");
    drop(logger);
    const PATH: &str = "output/log.txt";
    let file = File::open(PATH)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|res| res.unwrap())
        .collect();
    let comp = Ord::cmp(
        lines.as_slice(),
        &["Foobar.".to_string(), "Disposing the logger...".to_string()],
    );
    fs::remove_file(PATH)?;
    assert!(comp == Ordering::Equal);
    Ok(())
}

#[test]
fn store_succeeds() -> Result<(), Box<dyn Error>> {
    let _lock = LOCK.lock()?;
    let logger = Logger::new()?;
    store!(logger, 0, Severity::Info, "Foobar.");
    store!(logger, 0, Severity::Info, "Barbaz.");
    store!(logger, 1, Severity::Info, "Bazinga.");
    logger.commit(0);
    logger.flush();
    drop(logger);
    const PATH: &str = "output/log.txt";
    let file = File::open(PATH)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|res| res.unwrap())
        .collect();
    let comp = Ord::cmp(
        lines.as_slice(),
        &[
            "Foobar.".to_string(),
            "Barbaz.".to_string(),
            "Disposing the logger...".to_string(),
        ],
    );
    fs::remove_file(PATH)?;
    assert!(comp == Ordering::Equal);
    Ok(())
}

//TODO: test if sync succeeds