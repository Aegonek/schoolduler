// Gene: (index, value)
// Index represents course for which we are scheduling
// Value represents hour at which we will have class
// Chromosome: Vec<Gene> (for all lessons we need to schedule)
// We are searching for any viable solutions, that means, solution with least conflicts.

use crate::domain::*;
use bitvec::prelude::*;
use rand::prelude::*;

use crate::algen::parametrized::encoding::Decoder;
use crate::algen::parametrized::algorithm::Algorithm;
use crate::algen::parametrized::execution::History;
use crate::algen::parametrized::chromosome::IsChromosome;
use crate::algen::parametrized::config::IsConfig;
use crate::utils::units::Percent;
use self::config::Config;
use self::crossover_ops::one_point_crossover;
use self::fitness_ops::inverse_of_no_class_conflicts;
use self::mutation_ops::{creep_mutation, invert_bit_mutation};
use self::survivor_select_ops::roulette_selection;
use std::fmt::Display;
use std::ops::Range;
use crate::utils::{rated::Rated, units::Promile};
use derive_more::{AsMut, AsRef};
use rand::distributions::Uniform;

pub mod config;
pub mod encoding;
mod crossover_ops;
mod fitness_ops;
mod mutation_ops;
mod survivor_select_ops;

#[derive(Debug, Default, Clone, AsRef, AsMut)]
pub struct Chromosome(pub BitVec<u8>);

impl IsChromosome for Chromosome {
    type Index = usize;
    type Indices = Range<usize>;

    fn indices(&self) -> Self::Indices {
        0..self.0.len()
    }
}

impl Display for Chromosome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))?;
        Ok(())
    }
}

impl AsRef<[u8]> for Chromosome {
    fn as_ref(&self) -> &[u8] {
        self.0.as_raw_slice()
    }
}

pub struct Solution {
    courses: Vec<Course>,
    hours: Vec<LessonHour>,
    config: Config
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            courses: Vec::new(),
            hours: Vec::new(),
            config: Config::default()
        }
    }

    pub fn with_config(config: Config) -> Self {
        Solution {
            courses: Vec::new(),
            hours: Vec::new(),
            config: config
        }
    }
}

pub enum MutationOp {
    CreepMutation { from_distribution: Uniform<u8> },
    InvertBitMutation,
}

pub enum TerminationCondition {
    AfterNoIterations(usize),
}

impl Algorithm for Solution {
    type Chromosome = self::Chromosome;
    type Config = self::Config;

    fn config(&self) -> &Self::Config { &self.config }

    fn config_mut(&mut self) -> &mut Self::Config { &mut self.config }

    fn fitness_function(&self, chromosome: &Chromosome) -> u32 {
        inverse_of_no_class_conflicts(self, chromosome)
    }

    fn parent_selection_op(&self, population: &[Rated<Chromosome>])  
        -> (Rated<Chromosome>, Rated<Chromosome>) {
        let parents = (
            roulette_selection(population),
            roulette_selection(population),
        );
        parents
    }

    fn crossover_op(&self, lhs: Chromosome, rhs: Chromosome) -> (Chromosome, Chromosome) {
        one_point_crossover(lhs, rhs)
    }

    fn mutation_op(&self, genes: &mut Chromosome, i: usize) {
        use MutationOp::*;
        match self.config.mutation_op {
            CreepMutation { from_distribution } => {
                creep_mutation(from_distribution, &mut genes.0, i)
            }
            InvertBitMutation => invert_bit_mutation(&mut genes.0, i),
        }
    }

    fn survivor_selection_op(&self, population: &mut [Rated<Chromosome>]) -> Rated<Chromosome> {
        roulette_selection(population)
    }

    fn termination_condition(&self, history: &History<Self>) -> bool {
        use TerminationCondition::*;
        match self.config.termination_condition {
            AfterNoIterations(i) => {
                let front = history.0.front();
                match front {
                    Some(first) => first.iteration > i,
                    None => false 
                }
            }
        }
    }
}
