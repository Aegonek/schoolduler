use std::fmt::Display;

pub fn log_with_ctx<T: Display + DbWrite>(item: &T, ctx: T::Context) {
    eprintln!("{item}");
    if let Err(err) = item.write_db(ctx) {
        eprintln!("Error when trying to write value to DB! | {err}")
    }
}

pub fn log_annotated_with_ctx<T: Display + DbWrite>(item: &T, ctx: T::Context, annotation: &str) {
    eprintln!("{annotation} | {item}");
    if let Err(err) = item.write_db(ctx) {
        eprintln!("Error when trying to write value to DB! | {err}")
    }
}

pub fn log<T: Display + DbWrite<Context = ()>>(item: &T) { log_with_ctx(item, ()) }

pub fn log_annotated<T: Display + DbWrite<Context = ()>>(item: &T, annotation: &str) { log_annotated_with_ctx(item, (), annotation) }

pub trait DbWrite {
    type Context;

    fn write_db(&self, ctx: Self::Context) -> rusqlite::Result<()>;
}
