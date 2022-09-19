use itertools::izip;

use crate::logging::comm::Severity;

use super::{info, store, Logger};
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
    let lines: Vec<(String, String)> = BufReader::new(file)
        .lines()
        .map(|res| {
            let line = res.unwrap();
            let (time, rest) = line.split_once(" ").unwrap();
            (time.to_owned(), rest.to_owned())
        })
        .collect();
    // Dummy time, we only check format
    let expected = [("00:00:00", "[INFO]: Foobar."), ("00:00:00", "[INFO]: Disposing the logger...")];
    fs::remove_file(PATH)?;
    
    assert!(izip!(lines, expected).all(|(actual, expected)| actual.1 == expected.1 && actual.0.len() == expected.0.len()));
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
    let lines: Vec<(String, String)> = BufReader::new(file)
        .lines()
        .map(|res| {
            let line = res.unwrap();
            let (time, rest) = line.split_once(" ").unwrap();
            (time.to_owned(), rest.to_owned())
        })
        .collect();
    let expected = [
        ("00:00:00", "[INFO]: Foobar."),
        ("00:00:00", "[INFO]: Barbaz."),
        ("00:00:00", "[INFO]: Disposing the logger..."),
    ];
    fs::remove_file(PATH)?;
    
    assert!(izip!(lines, expected).all(|(actual, expected)| actual.1 == expected.1 && actual.0.len() == expected.0.len()));
    Ok(())
}

//TODO: test if sync succeeds