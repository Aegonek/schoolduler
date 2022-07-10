
use super::*;
use std::fmt::Display;
use once_cell::sync::Lazy;
use rusqlite::{self, params};
use time::OffsetDateTime;
use crate::DB_CONN;
use crate::utils::exts::result::ResultExt;
use crate::utils::log::DbWrite;

pub static LAST_RUN: Lazy<usize> = Lazy::new(|| {
    DB_CONN.lock().unwrap().query_row("
        SELECT run FROM THETA_ITERATIONS 
        ORDER BY run DESC
        LIMIT 1
    ", [], |row| row.get(0)).unwrap()
});

impl Display for Iteration<Solution> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Iteration: {}, best rating: {}, best chromosome: {}", self.iteration, self.best_result.rating, self.best_result.value.0))?;
        Ok(())
    }
}

impl DbWrite for Iteration<Solution> {
    fn write_db(&self) -> rusqlite::Result<()> {
        DB_CONN.lock().unwrap().execute("
            INSERT INTO THETA_ITERATIONS (run, iteration, rating, chromosome, time)
            VALUES (?1, ?2, ?3, ?4)
        ", params![
            *LAST_RUN + 1,
            self.iteration, 
            self.best_result.rating, 
            self.best_result.value.0.as_raw_slice(), OffsetDateTime::now_utc()
        ]).void()
    }
}