use crate::utils;
use once_cell::sync::OnceCell;
use std::error::Error;
use std::sync::Mutex;

pub mod comm;
mod logger;
pub mod handle;
#[cfg(test)]
mod tests;

static mut LOGGER: OnceCell<Mutex<handle::LogHandle>> = OnceCell::new();

pub fn logger() -> Logger {
    let logger = unsafe { LOGGER.get() }
        .take()
        .expect("Unexpected error: uninitialized logger!")
        .lock()
        .unwrap()
        .clone();
    logger
}

pub fn init_logger() -> Result<LoggerScope, Box<dyn Error>> {
    let logger = Logger::new()?;
    unsafe {
        LOGGER
            .set(Mutex::new(logger))
            .map_err(|_| utils::error::custom("Unexpected error: logger already initialized!"))
    }?;
    Ok(LoggerScope)
}

fn deinit_logger() -> Result<(), Box<dyn Error>> {
    let mutex = unsafe { LOGGER.take() }.ok_or(utils::error::custom(
        "Unexpected error: uninitialized logger!",
    ))?;
    let logger = mutex.into_inner()?;
    drop(logger);
    Ok(())
}

pub struct LoggerScope;
impl Drop for LoggerScope {
    fn drop(&mut self) {
        deinit_logger().unwrap()
    }
}

pub type Logger = handle::LogHandle;

pub type HashCode = u64;

pub use handle::{info, warning, error, store};
