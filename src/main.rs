#![feature(generic_associated_types, let_else)]
#![allow(dead_code)]

use rusqlite::Connection;
use once_cell::sync::Lazy;

mod algen;
mod domain;
mod utils;

const DB_PATH: &'static str = "data/schoolduler.db";
const DB_CONN: Lazy<Connection> = Lazy::new(|| {
    Connection::open("data/schoolduler.db").unwrap()
});

fn main() {
    println!("Forcing build?")
}


#[cfg(test)]
mod tests {
    use crate::DB_CONN;

    #[test]
    fn db_initialized() {
        let res = DB_CONN.query_row("SELECT name FROM sqlite_schema WHERE type='table' AND name='THETA_ITERATIONS'", [], |row| { Ok(()) });
        assert!(res.is_ok());
    }
}