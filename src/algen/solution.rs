pub mod crossover_ops;
pub mod fitness_ops;
pub mod mutation_ops;
pub mod select_ops;

use crate::school::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::borrow::Borrow;
use std::error::Error;
use std::ops::Range;

use super::config::{Config, self};
use super::encoding::Decoder;
use super::history::{Iteration, Leaderboard};
use super::random;
use super::Chromosome;
use crate::log::{log, Logger};
use crate::utils::probability::Probability::Promile;
use crate::utils::rating::{Rated, Rating};

pub struct Solution {
    pub decoder: Decoder,
    pub leaderboard: Leaderboard,
}

impl Solution {
    pub fn run(
        mut self,
        requirements: &Requirements,
        logger: &mut Logger,
    ) -> Result<Schedule, Box<dyn Error>> {
        const CONFIG: Config = config::CONFIG;
        const LOG_FREQUENCY: usize = 10;
        log!(logger, "Generating solution...");

        log!(logger, "Generating random schedules...");
        let courses: [Schedule; CONFIG.population_size] = [(); CONFIG.population_size]
            .map(|_| random::random_schedule(requirements));

        log!(logger, "Encoding and rating initial schedules...");
        let population: Vec<_> = courses
            .into_iter()
            .map(|crs| self.decoder.encode(&crs))
            .collect();
        let mut population: Vec<_> = population
            .into_iter()
            .map(|chrom| self.rated(chrom))
            .collect();

        let mut i = 0;
        log!(logger, "Starting the genetic algorithm!");
        while !self.should_terminate() {
            const NUMBER_OF_CHILDREN: usize = CONFIG.population_size * CONFIG.children_per_parent;

            let parents: Vec<_> = (0..NUMBER_OF_CHILDREN)
                .into_par_iter()
                .map(|_| self.select_parents(&population))
                .collect();

            let children: Vec<_> = parents
                .into_par_iter()
                .flat_map_iter(|(parent1, parent2)| {
                    let (mut child1, mut child2) = if Promile(thread_rng().gen_range(0..=1000))
                        <= CONFIG.crossover_probability
                    {
                        self.crossover(parent1.value.to_owned(), parent2.value.to_owned())
                    } else {
                        (parent1.value.to_owned(), parent2.value.to_owned())
                    };

                    self.mutate(&mut child1);
                    self.mutate(&mut child2);
                    [child1, child2]
                })
                .collect();
            let children: Vec<_> = children
                .into_iter()
                .map(|chrom| self.rated(chrom))
                .collect();

            let next_generation: Vec<_> = (0..CONFIG.population_size)
                .into_par_iter()
                .map(|_| self.select_survivor(&children).to_owned())
                .collect();
            population = next_generation;

            if i % LOG_FREQUENCY == 0 {
                let best = population.iter().max().unwrap();
                let iteration = Iteration {
                    iteration: i,
                    best_rating: best.rating,
                };
                log!(logger, "{}", iteration);
                self.leaderboard.iterations.push_front(iteration);
                if best.rating
                    > self
                        .leaderboard
                        .winner
                        .as_ref()
                        .map(|res| res.rating)
                        .unwrap_or(Rating(0))
                {
                    self.leaderboard.winner = Some(best.clone());
                }
            }
            i += 1;
        }

        let winner = self.leaderboard.winner.unwrap();
        log!(
            logger,
            "Finished running the algorithm! Best result is {}",
            winner.rating
        );
        let decoded = self.decoder.decode(&winner.value);
        log!(logger, "Generated solution!");
        return Ok(decoded);
    }

    pub fn rate(&mut self, chrom: &Chromosome) -> Rating {
        fitness_ops::inverse_of_no_class_conflicts(chrom, &mut self.leaderboard)
    }

    pub fn rated<T: Borrow<Chromosome>>(&mut self, chrom: T) -> Rated<T> {
        let rating = self.rate(chrom.borrow());
        Rated {
            value: chrom,
            rating,
        }
    }

    pub fn mutate(&self, chrom: &mut Chromosome) {
        // TODO: remove config as a param.
        // TODO: switch to creep mutation. After fixing compilation errors.
        mutation_ops::invert_bit_mutation(chrom, &config::CONFIG)
    }

    // Assuming always 2 parents and always 2 children during one crossover.
    pub fn crossover(&self, x: Chromosome, y: Chromosome) -> (Chromosome, Chromosome) {
        crossover_ops::one_point_crossover(x, y)
    }

    pub fn select_parents<'a>(
        &self,
        population: &'a [Rated<Chromosome>],
    ) -> (&'a Rated<Chromosome>, &'a Rated<Chromosome>) {
        let x = select_ops::roulette_selection(population);
        let y = select_ops::roulette_selection(population);
        (x, y)
    }

    pub fn select_survivor<'a>(
        &self,
        population: &'a [Rated<Chromosome>],
    ) -> &'a Rated<Chromosome> {
        select_ops::roulette_selection(population)
    }

    pub fn should_terminate(&self) -> bool {
        const NO_ITERATIONS: usize = 10000;
        match self.leaderboard.winner.as_ref() {
            Some(&Rated { rating, .. }) if rating == Rating::MAX => return true,
            _ => ()
        }

        self.leaderboard.iterations.front().map(|x| x.iteration).unwrap_or(0) >= NO_ITERATIONS
    }

    pub fn new() -> Self {
        Solution {
            decoder: Decoder::new(),
            leaderboard: Leaderboard::new(),
        }
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}
