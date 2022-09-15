use std::path::{Path, PathBuf};

use time::format_description::FormatItem;
use time::macros::format_description;
use time::OffsetDateTime;

const FMT: &[FormatItem] = format_description!("[day]-[month]_[hour]-[minute]-[second]");

pub fn timestamp_path<P: AsRef<Path>>(path: P, time: OffsetDateTime) -> PathBuf {
    let filename = path
        .as_ref()
        .file_name()
        .expect("Unexpected error: expecting that a path points to valid filename or directory.");
    let filename = filename
        .to_str()
        .expect("Unexpected error: expecting file name to contain only valid Unicode characters.");
    let new_filename = format!(
        "{}__{filename}",
        time.format(FMT)
            .expect("Unexpected error: expecting formatting to succeed.")
    );

    path.as_ref().with_file_name(new_filename)
}
