use super::{log, store, Logger};
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
    log!(logger, "Foobar.");
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
    store!(logger, 0, "Foobar.");
    store!(logger, 0, "Barbaz.");
    store!(logger, 1, "Bazinga.");
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
