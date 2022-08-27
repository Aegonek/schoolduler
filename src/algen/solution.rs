pub mod crossover_ops;
pub mod mutation_ops;
pub mod fitness_ops;
pub mod select_ops;
pub mod dispatch;

use rayon::prelude::*;
use rand::prelude::*;
use crate::domain::*;
use bitvec::vec::BitVec;
use std::error::Error;
use std::ops::Range;

use super::Chromosome;
use super::history::{Iteration, History};
use super::encoding::Decoder;
use super::params::*;
use super::random;
use crate::utils::log::{verbose, log, Logger};
use crate::utils::rated::{Rating, Rated};
use crate::utils::ratio::Promile;

#[derive(Default)]
pub struct Solution
{
    pub params: Params,
    pub adjust_strategy: AdjustStrategy,
    pub decoder: Decoder,
    pub fitness_function: FitnessFunction,
    pub parent_selection_op: ParentSelectionOp,
    pub crossover_op: CrossoverOp,
    pub mutation_op: MutationOp,
    pub survivor_selection_op: SurvivalSelectionOp,
    pub termination_condition: TerminationCondition,
}

// TODO: deserialization for configs for ease of testing and benchmarking different solutions
impl Solution
{
    pub fn run(mut self, requirements: &Requirements) -> Result<Schedule, Box<dyn Error>> {
        let mut logger = Logger::new()?;
        let mut history = History::new();
        
        log!(logger, "Generating random schedules...")?;
        let courses: Vec<Schedule> = vec![(); self.params.population_size]
            .into_par_iter()
            .map(|_| random::random_schedule(requirements))
            .collect();

        log!(logger, "Encoding and rating initial schedules...")?;
        let population: Vec<_> = courses.into_iter().map(|crs| self.decoder.encode(&crs)).collect();
        let mut population: Vec<_> = population.into_iter()
            .map(|chrom| self.rated(chrom))
            .collect();

        let mut i = 0;
        log!(logger, "Starting the genetic algorithm!")?;
        while !self.should_terminate(&history) {
            let no_children = self.params.population_size * self.params.children_per_parent;

            let parents: Vec<_> = (0..no_children)
                .into_par_iter()
                .map(|_| self.select_parents(&population))
                .collect();

            let children: Vec<_> = parents.into_par_iter()
                .flat_map_iter(|(parent1, parent2)| {
                    let (child1, child2) = if Promile(thread_rng().gen_range(0..=1000)) <= self.params.crossover_probability {
                        self.crossover(parent1.value.to_owned(), parent2.value.to_owned())
                    } else {
                        (parent1.value.to_owned(), parent2.value.to_owned())
                    };

                    [child1, child2].into_iter()
                        .map(|mut child| {
                            self.mutate(&mut child);
                            self.rated(child)
                        })
                })
                .collect();

            let next_generation: Vec<_> = (0..self.params.population_size)
                .into_par_iter()
                .map(|_| self.select_survivor(&children).to_owned())
                .collect();
            population = next_generation;

            if i % LOG_FREQUENCY == 0 {
                let best_rating = population.iter().max()
                    .unwrap().rating;
                let iteration = Iteration { iteration: i, best_rating };
                log!(logger, "{}", iteration)?;
                logger.log_benchmark(&iteration)?;
                history.0.push_front(iteration);
            }
            if i % self.params.adjustment_rate == 0 {
                self.adjust(&history);
            }
            i += 1;
        }

        let best_result = population.into_iter()
            .max().unwrap();
        log!(logger, "Finished running the algorithm! Best result is {}", best_result.rating)?;
        let decoded = self.decoder.decode(&best_result.value);
        return Ok(decoded);
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_params(params: Params) -> Self {
        Solution { params, ..Self::default() }
    }
}