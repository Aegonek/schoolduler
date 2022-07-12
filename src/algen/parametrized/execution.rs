use std::collections::VecDeque;
use std::fmt::Display;

use rusqlite::params;
use time::OffsetDateTime;

use crate::RUN_ID;
use crate::db::DB_CONN;
use crate::utils::exts::result::ResultExt;
use crate::utils::log::DbWrite;
use crate::utils::rated::Rated;
use super::algorithm::Algorithm;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunId(pub usize);

#[derive(Clone)]
pub struct Iteration<T>
where
    T: Algorithm,
{
    pub iteration: usize,
    pub best_result: Rated<T::Chromosome>,
}

impl<T: Algorithm> Display for Iteration<T>
where
    T::Chromosome: Display 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Iteration: {} | Best result: {}", self.iteration, self.best_result.rating))?;
        Ok(())
    }
}

impl<T: Algorithm> DbWrite for Iteration<T>
where
    T::Chromosome: AsRef<[u8]> {
    type Context = ();

    fn write_db(&self, _ctx: Self::Context) -> rusqlite::Result<()> {
        const SQL: &'static str = "
            INSERT INTO THETA_ITERATIONS (run, iteration, rating, time)
            VALUES (?1, ?2, ?3, ?4)
        ";
        DB_CONN.lock().unwrap()
        .execute(SQL, params![
            RUN_ID.get().unwrap().0,
            self.iteration, 
            self.best_result.rating,
            OffsetDateTime::now_utc()
        ]).void()
    }
}

#[derive(Clone)]
pub struct History<T: Algorithm>(/* used as stack */ pub VecDeque<Iteration<T>>);

impl<T: Algorithm> History<T> {
    pub fn new() -> Self {
        History(VecDeque::new())
    }
}