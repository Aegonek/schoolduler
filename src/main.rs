#![allow(dead_code)]

mod algen;
mod domain;
mod utils;

use std::fs;
use std::env;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("This argument was not valid path to .json files with requirements!");
    let _raw = String::from_utf8(fs::read(path)?)?;
    
    // println!("Reading input requirements...");
    // let courses: Vec<Course> = serde_json::from_str(&raw)?;
    // println!("Generating solution...");
    // let solver = theta::Solution::with_config(Config { 
    //     termination_condition: TerminationCondition::AfterNoIterations(100), 
    //     population_size: 30, 
    //     ..Config::default() 
    // });
    // let schedule = solver.run(&courses);

    // println!("Generated solution!");
    // xlsx::save_schedule(&schedule);

    Ok(())
}