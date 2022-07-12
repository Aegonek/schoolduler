
use rand::prelude::*;
use crate::domain::*;

use std::mem;
use tap::Pipe;

use crate::algen::random;
use crate::utils::exts::eager::EagerIter;
use crate::utils::rated::Rated;
use crate::utils::units::Promile;
use crate::utils::log::log_item;
use super::CALIBRATE_EVERY_N_ITERATIONS;
use super::chromosome::IsChromosome;
use super::config::IsConfig;
use super::encoding::Decoder;
use super::execution::{History, Iteration};

pub trait Algorithm
where
    Self: Sized,
    Self: Decoder<Encoded = Self::Chromosome>
{
    type Chromosome: IsChromosome; // Type representing one encoded solution
    type Config: IsConfig<Self>;

    // May change depending on ExecutionContext. This takes a snapshot.
    fn config(&self) -> &Self::Config;
    fn config_mut(&mut self) -> &mut Self::Config;

    fn fitness_function(&self, chromosome: &Self::Chromosome) -> u32;

    // We assume that crossover always yields 2 chromosomes for 2 parents.
    fn parent_selection_op(&self, population: &[Rated<Self::Chromosome>]) -> (Rated<Self::Chromosome>, Rated<Self::Chromosome>);

    fn crossover_op(&self, lhs: Self::Chromosome, rhs: Self::Chromosome) -> (Self::Chromosome, Self::Chromosome);

    fn mutation_op(&self, chromosome: &mut Self::Chromosome, i: <Self::Chromosome as IsChromosome>::Index);

    // Choose one survivor from population. May or may not remove it from population.
    fn survivor_selection_op(&self, population: &mut [Rated<Self::Chromosome>]) -> Rated<Self::Chromosome>;

    fn termination_condition(&self, history: &History<Self>) -> bool;

    // TODO: replace println! with conditionally compiled verbose!, blocked on crate feature? flag?
    fn run(mut self, requirements: &Requirements) -> Schedule {
        let mut history = History::new();

        println!("Generating random schedules...");
        let courses: Vec<Schedule> = {
            let config = self.config();
            vec![(); config.population_size()]
                .eager_map(|_| random::random_schedule(requirements))
        };

        println!("Encoding schedules...");
        let population: Vec<Self::Chromosome> = courses.eager_map(|crs| self.encode(&crs));
        println!("Rating schedules...");
        let mut population: Vec<Rated<Self::Chromosome>> = population.eager_map(|chrom| self.rate(chrom));

        let mut i_count: usize = 0;
        while !self.termination_condition(&history) {
            let config = self.config();
            let no_children = config.population_size() * config.children_per_parent();
            println!("Choosing parents...");
            let parents: Vec<_> = (0..no_children)
                .map(|_| self.parent_selection_op(&population))
                .collect();

            println!("Generating children...");
            let mut children: Vec<Rated<Self::Chromosome>> = Vec::with_capacity(no_children);
            for (parent1, parent2) in parents {
                let (child1, child2) = if Promile(thread_rng().gen_range(0..=1000))
                    <= config.crossover_probability()
                {
                    self.crossover_op(parent1.value, parent2.value)
                } else {
                    (parent1.value, parent2.value)
                };

                for mut child in [child1, child2] {
                    for i in child.indices() {
                        if Promile(thread_rng().gen_range(0..=1000))
                            <= config.mutation_probability()
                        {
                            self.mutation_op(&mut child, i);
                        }
                    }
                    let rated_child = self.rate(child);
                    children.push(rated_child);
                }
            }

            println!("Choosing next generation...");
            let mut next_generation: Vec<Rated<Self::Chromosome>> =
                Vec::with_capacity(config.population_size());
            for _ in 0..config.population_size() {
                let chosen = self.survivor_selection_op(&mut children);
                next_generation.push(chosen.clone());
            }

            population = next_generation;

            if i_count % CALIBRATE_EVERY_N_ITERATIONS == 0 {
                let best_result = population.iter().max().unwrap().clone();
                let iteration: Iteration<Self> = Iteration {
                    iteration: i_count,
                    best_result,
                };
                log_item(&iteration);
                history.0.push_front(iteration);
                {
                    mem::drop(config);
                    self.config_mut().adjust(&history);
                }
            }
            i_count += 1;
        }

        return population.into_iter()
            .max().unwrap()
            .value
            .pipe(|chrom| self.decode(&chrom))
            .clone();
    }
}

trait AlgorithmExt: Algorithm {
    fn rate(&self, chromosome: Self::Chromosome) -> Rated<Self::Chromosome> {
        let rating = self.fitness_function(&chromosome);
        Rated::new(chromosome, rating)
    }
}

impl<T: Algorithm> AlgorithmExt for T {}
