#![allow(dead_code)]

use std::error::Error;

use algen::parametrized::algorithm::Algorithm;
use algen::solutions::theta;
use domain::{Course, Schedule};
use utils::log::log;

mod algen;
mod domain;
mod db;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = include_str!("../data/example-courses.json");
    let courses: Vec<Course> = serde_json::from_str(raw)?;
    let solver = theta::Solution::new();
    let schedule = solver.run(&courses);

    println!("Generated solution:\n");
    for class in schedule {
        log(&class);
    }

    Ok(())
}

fn save_to_xlsx(_schedule: &Schedule) {
    println!("Writing to xlsx not yet implemented.")
}