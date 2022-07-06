// Gene: (index, value)
// Index represents course for which we are scheduling
// Value represents hour at which we will have class
// Chromosome: Vec<Gene> (for all lessons we need to schedule)
// We are searching for any viable solutions, that means, solution with least conflicts.

use crate::domain::*;
use crate::algen::genes::Genotype;
use bitvec::prelude::*;
use bitvec::ptr::Mut;
use derive_more::{AsRef, AsMut};

// mod crossover_ops;
// mod encoding;
// mod fitness_ops;
// mod mutation_ops;
// mod survivor_select_ops;

#[derive(Debug, Default, Clone, AsRef, AsMut)]
struct Chromosome(BitVec<u8>);

impl Genotype for Chromosome {
    type Gene<'a> = BitRef<'a, Mut, u8>;
    type Genes<'a> = BitVec<u8>;

    fn genes(&mut self) -> &mut Self::Genes<'_> {
        &mut self.0
    }
}

pub struct Solution {
    courses: Vec<Course>,
    hours: Vec<LessonHour>,
}

impl Solution {
    fn new() -> Self {
        Solution { courses: Vec::new(), hours: Vec::new() }
    }
}