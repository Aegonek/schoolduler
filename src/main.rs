#![allow(dead_code)]

use std::env;

mod school;
mod input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let requirements_file_path = args.first().expect("You have to provide input data to generate some lessons!");
    println!("Hello, world!");
}
