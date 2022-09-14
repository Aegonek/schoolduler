#![allow(dead_code)]

mod algen;
mod domain;
mod utils;
mod args;

use std::error::Error;

use args::Args;
use clap::Parser;
use ::time::OffsetDateTime;
use once_cell::sync::OnceCell;

use crate::algen::solution::Solution;
use crate::domain::*;
use crate::utils::log::log;
use crate::utils::log::Logger;
use crate::utils::xlsx;

fn main() -> Result<(), Box<dyn Error>> {
    START_TIME.set(OffsetDateTime::now_local()?).unwrap();
    let mut logger = Logger::new()?;
    let args = Args::parse();
    log!(logger, "Starting to generate courses at {}", start_time())?;

    log!(logger, "Loading requirements...")?;
    let courses: Vec<Course> = args.requirements()?;

    log!(logger, "Loading configuration...")?;
    let params = args.params()?;

    let solver = Solution::with_params(params);

    log!(logger, "Generating solution...")?;
    let schedule = solver.run(&courses, &mut logger)?;
    log!(logger, "Generated solution!")?;

    log!(logger, "Saving schedule to excel files...")?;
    xlsx::save_schedule(&schedule)?;

    log!(logger, "Finished saving schedules.")?;
    Ok(())
}

static START_TIME: OnceCell<OffsetDateTime> = OnceCell::new();

pub fn start_time() -> OffsetDateTime {
    *START_TIME.get().expect("Start time not registered!")
}
