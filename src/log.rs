use crate::algen::history::Iteration;
use std::error::Error;
use std::fmt::Arguments;
use std::fs::File;
use std::io::{self, Write};
use crate::utils;
use time::{Instant, OffsetDateTime};

pub struct Logger {
    start_instant: Instant,
    start_time: OffsetDateTime,
    last_benchmark: Option<Instant>,
    benchmark_file: File,
    log_file: File,
}

impl Logger {
    // TODO: write to files on another thread.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let start_time = OffsetDateTime::now_local()?;
        let start_instant = Instant::now();

        let benchmark_path = utils::time::timestamp_path("output/benchmarks.csv", start_time);
        let mut benchmark_file = utils::fs::create_file_all(benchmark_path)?;
        writeln!(benchmark_file, "Seconds from last benchmark; Iteration; Best rating")?;

        let log_path = utils::time::timestamp_path("output/logs.txt", start_time);
        let log_file = utils::fs::create_file_all(log_path)?;

        let logger = Logger {
            start_time,
            start_instant,
            last_benchmark: None,
            benchmark_file,
            log_file,
        };
        Ok(logger)
    }

    pub fn log(&mut self, args: Arguments) -> Result<(), io::Error> {
        println!("{args}");
        writeln!(self.log_file, "{args}")
    }

    pub fn log_benchmark(&mut self, iteration: &Iteration) -> Result<(), io::Error> {
        writeln!(
            self.benchmark_file,
            "{:.2} ; {} ; {}",
            self.last_benchmark.unwrap_or(self.start_instant).elapsed().as_seconds_f64(),
            iteration.iteration,
            iteration.best_rating
        )?;
        self.last_benchmark = Some(Instant::now());
        Ok(())
    }

    #[inline]
    pub fn start_time(&self) -> OffsetDateTime { self.start_time }
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $($x:tt)*) => {
            $logger.log(format_args!($($x)*))
    };
}

pub use log;
