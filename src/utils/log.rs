use std::fmt::Display;

pub fn log_item_with_ctx<T: Display + DbWrite>(item: &T, ctx: T::Context) {
    println!("{item}");
    if let Err(err) = item.write_db(ctx) {
        println!("Error when trying to write value to DB! | {err}")
    }
}

pub fn log_item_annotated_with_ctx<T: Display + DbWrite>(item: &T, ctx: T::Context, annotation: &str) {
    println!("{annotation} | {item}");
    if let Err(err) = item.write_db(ctx) {
        println!("Error when trying to write value to DB! | {err}")
    }
}

pub fn log_item<T: Display + DbWrite<Context = ()>>(item: &T) { log_item_with_ctx(item, ()) }

pub fn log_item_annotated<T: Display + DbWrite<Context = ()>>(item: &T, annotation: &str) { log_item_annotated_with_ctx(item, (), annotation) }

pub trait DbWrite {
    type Context;

    fn write_db(&self, ctx: Self::Context) -> rusqlite::Result<()>;
}
