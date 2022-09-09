use std::path::{Path, PathBuf};

use time::format_description::FormatItem;
use time::macros::format_description;

use crate::start_time;

const TIME_FORMAT: &[FormatItem] = format_description!("[day]_[month]__[hour]_[minute]_[second]");

pub fn timestamped<P: AsRef<Path>>(path: P) -> PathBuf {
    let filename = path
        .as_ref()
        .file_name()
        .expect("Expecting that a path points to valid filename or directory.");
    let filename = filename
        .to_str()
        .expect("Expecting file name to contain only valid Unicode characters.");
    let new_filename = format!(
        "{}__{filename}",
        start_time()
            .format(TIME_FORMAT)
            .expect("Expecting formatting to succeed lol.")
    );

    path.as_ref().with_file_name(new_filename)
}
