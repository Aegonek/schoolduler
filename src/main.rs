#![feature(drain_filter)]
#![allow(dead_code)]

use std::slice;
use std::{env, iter};

mod school;
mod input;
mod algen;

fn main() {
    let requirements = input::mock_requirements();
}