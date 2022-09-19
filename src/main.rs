use std::error::Error;

use clap::Parser;

use schoolduler::args::Args;
use schoolduler::algen::solution::Solution;
use schoolduler::school::*;
use schoolduler::logging::{self, info, error, logger};
use schoolduler::xlsx;

fn main() -> Result<(), Box<dyn Error>> {
    match _main() {
        Ok(_) => Ok(()),
        Err(err) => {
            error!(logger(), "Fatal error: {}", err);
            logging::deinit_logger().unwrap();
            Err(err)
        }
    }
}

fn _main() -> Result<(), Box<dyn Error>> {
    logging::init_logger()?;
    let args = Args::parse();
    let logger = logger();
    info!(logger, "Starting the application at {}", logger.start_time());

    let courses: Vec<Course> = args.requirements()?;

    let solver = Solution::new();
    let schedule = solver.run(&courses)?;
    xlsx::save_schedule(&schedule)?;
    logging::deinit_logger()?;
    Ok(())
}
