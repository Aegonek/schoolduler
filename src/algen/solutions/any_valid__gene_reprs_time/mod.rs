// Gene: (index, value)
// Index represents course for which we are scheduling
// Value represents hour at which we will have class
// Chromosome: Vec<Gene> (for all lessons we need to schedule)
// We are searching for any viable solutions, that means, solution with least conflicts.

use crate::algen::encoding::Decoder;
use crate::domain::*;
use once_cell::sync::Lazy;
use rand::distributions::Uniform;
use rand::prelude::*;

mod crossover_ops;
mod encoding;
mod fitness_ops;
mod mutation_ops;
mod survivor_select_ops;

type Gene = u8;
type Chromosome = Vec<u8>;

const MUTATION_PROBABILITY: u32 = 5;
const MUTATION_CREEP_DISTRIBUTION: Lazy<Uniform<i32>> = Lazy::new(|| Uniform::new_inclusive(-5, 5));

pub struct Solution {
    courses: Vec<Course>,
    hours: Vec<LessonHour>,
}

impl Solution {
    fn new() -> Self {
        Solution { courses: Vec::new(), hours: Vec::new() }
    }
}