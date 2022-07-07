// Gene: (index, value)
// Index represents course for which we are scheduling
// Value represents hour at which we will have class
// Chromosome: Vec<Gene> (for all lessons we need to schedule)
// We are searching for any viable solutions, that means, solution with least conflicts.

use crate::algen::algorithm::{self, Algorithm};
use crate::utils::units::Promile;
use crate::{domain::*, algen::execution::ExecutionContext};
use crate::algen::genes::Genotype;
use bitvec::prelude::*;
use bitvec::ptr::Mut;
use derive_more::{AsRef, AsMut};
use rand::distributions::Uniform;

// mod crossover_ops;
mod mutation_ops;
mod encoding;
// mod fitness_ops;
// mod survivor_select_ops;

#[derive(Debug, Default, Clone, AsRef, AsMut)]
pub struct Chromosome(pub BitVec<u8>);

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
    config: Config,
    execution_context: ExecutionContext<Self>
}

impl Solution {
    fn new() -> Self {
        Solution { 
            courses: Vec::new(), 
            hours: Vec::new(),
            config: Config::default(),
            execution_context: ExecutionContext::default()
        }
    }
}

#[non_exhaustive]
pub enum MutationOp {
    CreepMutation { from_distribution: Uniform<u8> },
    InvertBitMutation
}

pub struct Config {
    pub population_size: usize,
    // probability that a gene will be mutated
    pub mutation_probability: Promile,
    // probability that a chromosome will be crossovered with another chromosome
    pub crossover_probability: Promile,
    pub children_per_parent: usize,
    pub mutation_op: MutationOp,
}

impl Default for Config {
    fn default() -> Self {
        todo!()
    }
}

impl From<&Config> for algorithm::Config {
    fn from(cfg: &Config) -> Self {
        algorithm::Config { 
            population_size: cfg.population_size, 
            mutation_probability: cfg.mutation_probability, 
            crossover_probability: cfg.crossover_probability, 
            children_per_parent: cfg.children_per_parent 
        }
    }
}

impl Algorithm for Solution {
    type Chromosome = self::Chromosome;

    fn config(&self) -> algorithm::Config {
        todo!()
    }

    fn execution_context(&mut self) -> &mut ExecutionContext<Self> {
        todo!()
    }

    fn fitness_function(&self, chromosome: &Self::Chromosome) -> u32 {
        todo!()
    }

    fn parent_selection_op(&self, population: &[crate::utils::rated::Rated<Self::Chromosome>]) -> (Self::Chromosome, Self::Chromosome) {
        todo!()
    }

    fn crossover_op(&self, lhs: Self::Chromosome, rhs: Self::Chromosome) -> (Self::Chromosome, Self::Chromosome) {
        todo!()
    }

    fn mutation_op(&self, genes: &mut <Self::Chromosome as Genotype>::Genes<'_>, i: usize) {
        todo!()
    }

    fn survivor_selection_op(&self, population: &mut [crate::utils::rated::Rated<Self::Chromosome>],) -> crate::utils::rated::Rated<Self::Chromosome> {
        todo!()
    }

    fn termination_condition(&self) -> bool {
        todo!()
    }
}