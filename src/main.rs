#![allow(dead_code)]

pub mod algen;
pub mod domain;
pub mod args;
pub mod xlsx;
pub mod log;
pub mod utils;

use std::error::Error;

use args::Args;
use clap::Parser;
use ::time::OffsetDateTime;
use once_cell::sync::OnceCell;

use crate::algen::solution::Solution;
use crate::domain::*;
use crate::log::{log, Logger};

fn main() -> Result<(), Box<dyn Error>> {
    START_TIME.set(OffsetDateTime::now_local()?).unwrap();
    let mut logger = Logger::new()?;
    let args = Args::parse();
    log!(logger, "Starting the application at {}", start_time())?;

    let courses: Vec<Course> = args.requirements(&mut logger)?;
    let params = args.params(&mut logger)?;

    let solver = Solution::with_params(params);
    let schedule = solver.run(&courses, &mut logger)?;
    xlsx::save_schedule(&schedule, &mut logger)?;
    Ok(())
}

static START_TIME: OnceCell<OffsetDateTime> = OnceCell::new();

pub fn start_time() -> OffsetDateTime {
    *START_TIME.get().expect("Start time not registered!")
}
