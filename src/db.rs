use rusqlite::Connection;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// TODO: make thread local, drop mutexes?
static DB_PATH: &'static str = "output/schoolduler.db";
pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open(DB_PATH).unwrap();
    Mutex::new(conn)
});

#[cfg(test)]
mod tests {
    use super::DB_CONN;

    #[test]
    fn db_initialized() {
        let res = DB_CONN.lock().unwrap().query_row("SELECT name FROM sqlite_schema WHERE type='table' AND name='THETA_ITERATIONS'", [], |_| { Ok(()) });
        assert!(res.is_ok());
    }
}