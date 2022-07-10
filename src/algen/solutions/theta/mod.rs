// Gene: (index, value)
// Index represents course for which we are scheduling
// Value represents hour at which we will have class
// Chromosome: Vec<Gene> (for all lessons we need to schedule)
// We are searching for any viable solutions, that means, solution with least conflicts.

use std::fmt::Display;
use std::ops::Range;

use crate::domain::*;
use crate::utils::units::Percent;
use bitvec::prelude::*;
use rand::prelude::*;

use self::crossover_ops::one_point_crossover;
use self::fitness_ops::inverse_of_no_class_conflicts;
use self::mutation_ops::{creep_mutation, invert_bit_mutation};
use self::survivor_select_ops::roulette_selection;
use crate::algen::algorithm::{self, Algorithm, IsChromosome};
use crate::algen::encoding::Decoder;
use crate::algen::execution::{History, Iteration};
use crate::utils::{rated::Rated, units::Promile};
use derive_more::{AsMut, AsRef};
use rand::distributions::Uniform;

mod crossover_ops;
mod encoding;
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
    config: Config,
    history: History<Self>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            courses: Vec::new(),
            hours: Vec::new(),
            config: Config::default(),
            history: History::new(),
        }
    }
}

pub struct Config {
    pub population_size: usize,
    // probability that a gene will be mutated
    pub mutation_probability: Promile,
    // probability that a chromosome will be crossovered with another chromosome
    pub crossover_probability: Promile,
    pub children_per_parent: usize,
    pub mutation_op: MutationOp,
    pub termination_condition: TerminationCondition,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            population_size: 1000,
            mutation_probability: Percent(5).into(),
            crossover_probability: Percent(30).into(),
            children_per_parent: 1,
            mutation_op: MutationOp::InvertBitMutation,
            termination_condition: TerminationCondition::AfterNoIterations(100_000),
        }
    }
}

impl From<&Config> for algorithm::Config {
    fn from(cfg: &Config) -> Self {
        algorithm::Config {
            population_size: cfg.population_size,
            mutation_probability: cfg.mutation_probability,
            crossover_probability: cfg.crossover_probability,
            children_per_parent: cfg.children_per_parent,
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

    fn config(&self) -> algorithm::Config {
        algorithm::Config::from(&self.config)
    }

    fn history(&mut self) -> &mut History<Self> {
        &mut self.history
    }

    fn fitness_function(&self, chromosome: &Chromosome) -> u32 {
        inverse_of_no_class_conflicts(self, chromosome)
    }

    fn parent_selection_op(
        &self,
        population: &[Rated<Chromosome>],
    ) -> (Rated<Chromosome>, Rated<Chromosome>) {
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

    fn termination_condition(&self) -> bool {
        use TerminationCondition::*;
        match self.config.termination_condition {
            AfterNoIterations(i) => {
                let front = self.history.0.front();
                match front {
                    Some(first) => first.iteration > i,
                    None => false 
                }
            }
        }
    }
}
