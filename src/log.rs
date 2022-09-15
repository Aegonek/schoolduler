use crate::algen::history::Iteration;
use std::error::Error;
use std::fmt::Arguments;
use std::fs::File;
use std::io::{self, Write};
use time::macros::format_description;
use time::{Instant, OffsetDateTime};

pub struct Logger {
    start: Instant,
    last_benchmark: Option<Instant>,
    benchmark_file: File,
    log_file: File,
}

impl Logger {
    // TODO: write to files on another thread.
    // TODO: lock stdout when writing.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let now = OffsetDateTime::now_local()?;
        let time_format = format_description!("[day]_[month]__[hour]_[minute]_[second]");
        let start = Instant::now();

        let benchmark_file = format!("output/{}__benchmarks.csv", now.format(time_format)?);
        let mut benchmark_file = File::create(benchmark_file)?;
        writeln!(benchmark_file, "Seconds from last benchmark ; Iteration ; Best rating")?;

        let log_file = format!("output/{}__logs.csv", now.format(time_format)?);
        let log_file = File::create(log_file)?;

        let logger = Logger {
            start,
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
            self.last_benchmark.unwrap_or(self.start).elapsed().as_seconds_f64(),
            iteration.iteration,
            iteration.best_rating
        )?;
        self.last_benchmark = Some(Instant::now());
        Ok(())
    }
}

macro_rules! log {
    ($logger:expr, $($x:tt)*) => {
            $logger.log(format_args!($($x)*))
    };
}

pub(crate) use log;