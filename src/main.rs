use std::error::Error;

use clap::Parser;

use schoolduler::args::Args;
use schoolduler::algen::solution::Solution;
use schoolduler::school::*;
use schoolduler::log::{self, log_fmt, logger};
use schoolduler::xlsx;

fn main() -> Result<(), Box<dyn Error>> {
    log::init_logger()?;
    let args = Args::parse();
    let logger = logger();
    log_fmt!(logger, "Starting the application at {}", logger.start_time());

    let courses: Vec<Course> = args.requirements()?;

    let solver = Solution::new();
    let schedule = solver.run(&courses)?;
    xlsx::save_schedule(&schedule)?;
    log::deinit_logger()?;
    Ok(())
}
