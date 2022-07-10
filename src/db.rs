use rusqlite::Connection;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::algen::parametrized::execution::RunId;

static DB_PATH: &'static str = "data/schoolduler.db";
pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("data/schoolduler.db").unwrap();
    Mutex::new(conn)
});

// TODO: test
pub static RUN_ID: Lazy<RunId> = Lazy::new(|| {
    let nmb = DB_CONN.lock().unwrap().query_row("
        SELECT run FROM THETA_ITERATIONS 
        ORDER BY run DESC
        LIMIT 1
    ", [], |row| row.get(0)).unwrap();
    RunId(nmb)
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