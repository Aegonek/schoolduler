use std::{fs, path::Path};
use rusqlite::{Connection, Result};

// TODO - use some basic migration system.
fn main() -> Result<()> {
    if !Path::new("output").exists() {
        fs::create_dir("output").unwrap();
    }

    let conn = Connection::open("output/schoolduler.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS THETA_ITERATIONS (
            id INTEGER PRIMARY KEY,
            run INTEGER NOT NULL,
            iteration INTEGER NOT NULL,
            rating INTEGER NOT NULL,
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