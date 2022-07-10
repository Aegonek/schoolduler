use std::fmt::Display;

pub fn log<T: Display + DbWrite>(item: &T, ctx: T::Context) {
    eprintln!("{item}");
    if let Err(err) = item.write_db(ctx) {
        eprintln!("Error when trying to write value to DB! {err}")
    }
}

pub fn log_annotated<T: Display + DbWrite>(item: &T, ctx: T::Context, annotation: &str) {
    eprintln!("{annotation} | {item}");
    if let Err(err) = item.write_db(ctx) {
        eprintln!("Error when trying to write value to DB! {err}")
    }
}

pub trait DbWrite {
    type Context;

    fn write_db(&self, ctx: Self::Context) -> rusqlite::Result<()>;
}
