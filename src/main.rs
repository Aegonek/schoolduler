#![feature(drain_filter, exclusive_range_pattern)]
#![allow(dead_code)]

mod school;
mod input;
mod algen;

use input::mockups;

fn main() {
    let requirements = mockups::mock_requirements();
    let solved = algen::solve(requirements);
}