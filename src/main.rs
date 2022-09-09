#![allow(dead_code)]

mod algen;
mod domain;
mod utils;

use std::fs;
use std::env;
use std::error::Error;

use once_cell::sync::OnceCell;
use ::time::OffsetDateTime;

use crate::algen::params::Params;
use crate::algen::params::TerminationCondition;
use crate::algen::solution::Solution;
use crate::domain::*;
use crate::utils::xlsx;

static START_TIME: OnceCell<OffsetDateTime> = OnceCell::new();

pub fn start_time() -> OffsetDateTime {
    *START_TIME.get().expect("Start time not registered!")
}

fn main() -> Result<(), Box<dyn Error>> {
    START_TIME.set(OffsetDateTime::now_local()?).unwrap();
    let path = if cfg!(feature = "debug") {
        env::args().nth(1).unwrap_or(r"C:\Users\domin\Projects\schoolduler\input\example-courses.json".to_owned())
    } else {
        env::args().nth(1).expect("This argument was not valid path to .json files with requirements!")
    };
    
    let raw = String::from_utf8(fs::read(path)?)?;
    
    println!("Reading input requirements...");
    let courses: Vec<Course> = serde_json::from_str(&raw)?;
    println!("Generating solution...");
    let solver = Solution::with_params(Params { 
        population_size: 30,
        termination_condition: TerminationCondition::AfterNoIterations(100),
        ..Params::default() 
    });
    let schedule = solver.run(&courses)?;

    println!("Generated solution!");

    println!("Saving schedule to excel files...");
    xlsx::save_schedule(&schedule)?;

    println!("Finished saving schedules.");
    Ok(())
}

#[test]
fn gen_random_schedule() -> Result<(), Box<dyn Error>> {
    let path = r"C:\Users\domin\Projects\schoolduler\input\example-courses.json".to_owned();
    let raw = String::from_utf8(fs::read(path)?)?;
    let courses: Vec<Course> = serde_json::from_str(&raw)?;
    let solution = algen::random::random_schedule(&courses);
    let serialized = serde_json::to_string(&solution)?;
    fs::write(r"C:\Users\domin\Projects\schoolduler\input\random-schedule.json", serialized)?;
    Ok(())
}