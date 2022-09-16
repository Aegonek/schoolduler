use std::error::Error;

use clap::Parser;

use schoolduler::args::Args;
use schoolduler::algen::solution::Solution;
use schoolduler::school::*;
use schoolduler::log::{log, Logger};
use schoolduler::xlsx;

fn main() -> Result<(), Box<dyn Error>> {
    let mut logger = Logger::new()?;
    let args = Args::parse();
    log!(logger, "Starting the application at {}", logger.start_time())?;

    let courses: Vec<Course> = args.requirements(&mut logger)?;
    let params = args.params(&mut logger)?;

    let solver = Solution::with_params(params);
    let schedule = solver.run(&courses, &mut logger)?;
    xlsx::save_schedule(&schedule, &mut logger)?;
    Ok(())
}
