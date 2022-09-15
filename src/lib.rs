pub mod algen;
pub mod domain;
pub mod xlsx;
pub mod args;
pub mod log;
pub mod utils;

use once_cell::sync::OnceCell;
use time::OffsetDateTime;

pub static START_TIME: OnceCell<OffsetDateTime> = OnceCell::new();

pub fn start_time() -> OffsetDateTime {
    *START_TIME.get().expect("Start time not registered!")
}
