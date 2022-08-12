use std::error::Error;
use std::fmt::Arguments;
use std::fs::File;
use std::io::{Write, self};
use time::{OffsetDateTime, Instant};
use time::macros::format_description;
use crate::algen::parametrized::history::Iteration;

pub struct Logger {
    start: Instant,
    benchmark_file: File,
    log_file: File
}

impl Logger {
    pub fn new()-> Result<Self, Box<dyn Error>> {
        let now = OffsetDateTime::now_local()?;
        let time_format = format_description!("[day]_[month]__[hour]_[minute]_[second]");
        let start = Instant::now();

        let benchmark_file = format!("output/{}__benchmarks.csv", now.format(time_format)?);
        let mut benchmark_file = File::create(benchmark_file)?;
        writeln!(benchmark_file, "TIME_ELAPSED ; ITERATION ; BEST_RATING")?;

        let log_file = format!("output/{}__logs.csv", now.format(time_format)?);
        let log_file = File::create(log_file)?;
        
        let logger = Logger { start, benchmark_file, log_file };
        Ok(logger)
    }

    pub fn log(&mut self, args: Arguments) -> Result<(), io::Error> {
        writeln!(self.log_file, "{args}")
    }

    pub fn log_benchmark(&mut self, iteration: &Iteration) -> Result<(), io::Error> {
        writeln!(self.benchmark_file, "{} ; {} ; {}", self.start.elapsed(), iteration.iteration, iteration.best_rating)
    }
}

// TODO: ask on stack overflow why does assigning format_args! result to a variable cause "temporary value dropped when borrowed" error.
macro_rules! log {
    ($logger:expr, $($x:tt)*) => {
            $logger.log(format_args!($($x)*))
    };
}

pub(crate) use log;

macro_rules! verbose {
    ($($x:tt)*) => {
        #[cfg(feature = "verbose")]
        println!($($x)*);
    }
}

pub(crate) use verbose;