use crate::{domain::*, utils::indexed::Len};
use rand::prelude::*;
use std::mem;
use super::{execution::{ExecutionContext, Iteration}, encoding::Decoder, genes::Genotype, random};
use crate::utils::{eager::EagerIter, rated::Rated, units::Promile};
use tap::Pipe;

const LOG_EVERY_N_ITERATIONS: usize = 50;

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    population_size: usize,
    // probability that a gene will be mutated
    mutation_probability: Promile,
    // probability that a chromosome will be crossovered with another chromosome
    crossover_probability: Promile,
    children_per_parent: usize,
}

pub trait Algorithm
where
    Self: Sized,
    Self: Decoder<Encoded = Self::Chromosome>,
{
    type Chromosome: Genotype + Sized; // Type representing one encoded solution

    // May change depending on ExecutionContext. This takes a snapshot.
    fn config(&self) -> Config;

    fn execution_context(&mut self) -> &mut ExecutionContext<Self>;

    fn fitness_function(&self, chromosome: &Self::Chromosome) -> u32;

    // We assume that crossover always yields 2 chromosomes for 2 parents.
    fn parent_selection_op(&self, population: &[Rated<Self::Chromosome>]) -> (Self::Chromosome, Self::Chromosome);

    fn crossover_op(&self, lhs: Self::Chromosome, rhs: Self::Chromosome) -> (Self::Chromosome, Self::Chromosome);

    fn mutation_op(&self, genes: &mut <Self::Chromosome as Genotype>::Genes<'_>, i: usize);

    // Choose one survivor from population. May or may not remove it from population.
    fn survivor_selection_op(&self, population: &mut [Rated<Self::Chromosome>],) -> Rated<Self::Chromosome>;

    fn termination_condition(&self) -> bool;

    fn run(mut self, requirements: &Requirements) -> Schedule {
        let mut config = self.config();
        let courses: Vec<Schedule> = Vec::with_capacity(config.population_size as usize)
            .eager_map(|()| random::random_schedule(requirements));

        let population: Vec<Self::Chromosome> = courses.eager_map(|crs| self.encode(&crs));
        let mut population: Vec<Rated<Self::Chromosome>> = population.eager_map(|chrom| self.rate(chrom));
        while !self.termination_condition() {
            let no_children = config.population_size * config.children_per_parent;
            let parents: Vec<_> = (0..no_children)
                .map(|_| self.parent_selection_op(&population))
                .collect();

            let mut children: Vec<Rated<Self::Chromosome>> = Vec::with_capacity(no_children as usize);
            for (parent1, parent2) in parents {
                let (child1, child2) = if Promile(thread_rng().gen_range(0..=1000))
                    <= self.config().crossover_probability
                {
                    self.crossover_op(parent1, parent2)
                } else {
                    (parent1, parent2)
                };

                for mut child in [child1, child2] {
                    let genes = child.genes();
                    for i in 0..genes.len() {
                        if Promile(thread_rng().gen_range(0..=1000))
                            <= self.config().mutation_probability
                        {
                            self.mutation_op(genes, i);
                        }
                    }
                    let rated_child = self.rate(child);
                    children.push(rated_child);
                }
            }

            let mut next_generation: Vec<Rated<Self::Chromosome>> =
                Vec::with_capacity(config.population_size);
            for _ in 0..config.population_size {
                let chosen = self.survivor_selection_op(&mut children);
                next_generation.push(chosen);
            }

            config = self.config();
            let _ = mem::replace(&mut population, next_generation);

            let mut execution_context = self.execution_context();
            execution_context.iteration_count += 1;
            if execution_context.iteration_count % LOG_EVERY_N_ITERATIONS == 0 {
                let best_result = population.iter().max().unwrap().clone();
                let iteration: Iteration<Self> = Iteration {
                    iteration: execution_context.iteration_count,
                    best_result,
                };
                execution_context.history.push(iteration);
            }
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
