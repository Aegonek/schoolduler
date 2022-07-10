#![feature(generic_associated_types, let_else)]
#![allow(dead_code)]

use rusqlite::Connection;
use std::sync::Mutex;
use once_cell::sync::Lazy;

mod algen;
mod domain;
mod utils;

static DB_PATH: &'static str = "data/schoolduler.db";
pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("data/schoolduler.db").unwrap();
    Mutex::new(conn)
});

fn main() {
    println!("Forcing build?")
}


#[cfg(test)]
mod tests {
    use crate::DB_CONN;

    #[test]
    fn db_initialized() {
        let res = DB_CONN.lock().unwrap().query_row("SELECT name FROM sqlite_schema WHERE type='table' AND name='THETA_ITERATIONS'", [], |_| { Ok(()) });
        assert!(res.is_ok());
    }
}