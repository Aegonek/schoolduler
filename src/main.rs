#![allow(dead_code)]

mod algen;
mod domain;
mod utils;

use std::fs;
use std::env;
use std::error::Error;

use time::Instant;
use time::OffsetDateTime;
use time::macros::format_description;

use crate::algen::params::Params;
use crate::algen::params::TerminationCondition;
use crate::algen::solution::Solution;
use crate::domain::*;
use crate::utils::xlsx;


fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("This argument was not valid path to .json files with requirements!");
    let raw = String::from_utf8(fs::read(path)?)?;
    
    println!("Reading input requirements...");
    let courses: Vec<Course> = serde_json::from_str(&raw)?;
    println!("Generating solution...");
    let mut solver = Solution::with_params(Params { 
        population_size: 30, 
        ..Params::default() 
    });
    solver.termination_condition = TerminationCondition::AfterNoIterations(1000);
    let schedule = solver.run(&courses)?;

    println!("Generated solution!");

    // TODO: move to util
    let now = OffsetDateTime::now_local()?;
    let time_format = format_description!("[day]_[month]__[hour]_[minute]_[second]");

    let output_file = format!("output/{}__schedule.xlsx", now.format(time_format)?);
    xlsx::save_schedule(output_file, &schedule);

    Ok(())
}