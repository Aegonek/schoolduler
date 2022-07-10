use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("data/schoolduler.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS THETA_ITERATIONS (
            id INTEGER PRIMARY KEY,
            run INTEGER NOT NULL,
            iteration INTEGER NOT NULL,
            rating INTEGER NOT NULL,
            chromosome BLOB NOT NULL,
            time TEXT NOT NULL
        )",
        []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS THETA_RESULTS (
            id INTEGER PRIMARY KEY,
            run INTEGER NOT NULL,
            -- stored as json
            lesson TEXT NOT NULL
        )",
        []
    )?;

    Ok(())
}