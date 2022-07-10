use std::fmt::Display;

pub fn log<T: Display + DbWrite>(item: &T) {
    eprintln!("{item}");
    if let Err(err) = item.write_db() {
        eprintln!("Error when trying to write value to DB! {err}")
    }
}

pub fn log_annotated<T: Display + DbWrite>(item: &T, annotation: &str) {
    eprintln!("{annotation} | {item}");
    if let Err(err) = item.write_db() {
        eprintln!("Error when trying to write value to DB! {err}")
    }
}

pub trait DbWrite {
    fn write_db(&self) -> rusqlite::Result<()>;
}