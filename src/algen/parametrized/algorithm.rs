use rayon::prelude::*;
use rand::prelude::*;
use crate::algen::parametrized::history::Iteration;
use crate::domain::*;
use crate::algen::random;
use crate::utils::rated::{Rating, Rated};
use crate::utils::ratio::Promile;
use super::encoding::Decoder;
use super::history::History;
use super::params::Params;

// Update history once every LOG_FREQUENCY iterations
const LOG_FREQUENCY: usize = 10;

struct Algorithm<Chromosome> where
    Chromosome: Send + Sync 
{
    params: Params,
    adjust: fn(&mut Self, &History),
    decoder: Box<dyn Decoder<Encoded = Chromosome>>,
    fitness_function: fn(&Chromosome) -> Rating,
    // We assume that crossover always yields 2 chromosomes for 2 parents
    parent_selection_op: fn(&[Rated<Chromosome>]) -> (Rated<Chromosome>, Rated<Chromosome>),
    crossover_op: fn(Chromosome, Chromosome) -> (Chromosome, Chromosome),
    // Mutation must be applied to every gene, so we must pass params to mutation_op
    mutation_op: fn(&mut Chromosome, &Params),
    survivor_selection_op: fn(&[Rated<Chromosome>]) -> Rated<Chromosome>,
    termination: fn(&History) -> bool, 
}

impl<Chromosome> Algorithm<Chromosome> where
    Chromosome: Send + Sync 
{
    // TODO: verbose! macro for logs other than iteration
    // TODO: log iterations to .csv file.
    fn run(mut self, requirements: &Requirements) -> Schedule {
        let mut history = History::new();
        
        println!("Generating random schedules...");
        let courses: Vec<Schedule> = vec![(); self.params.population_size]
            .into_par_iter()
            .map(|_| random::random_schedule(requirements))
            .collect();

        println!("Encoding and rating initial schedules...");
        let population = courses.into_iter().map(|crs| self.decoder.encode(&crs));
        let mut population: Vec<Rated<Chromosome>> = population
            .map(|chrom| {
                let rating = (self.fitness_function)(&chrom);
                Rated { value: chrom, rating }
            })
            .collect();
        let mut i = 0;
        while !(self.termination)(&history) {
            let no_children = self.params.population_size * self.params.children_per_parent;

            println!("Choosing parents...");
            let parents: Vec<_> = (0..no_children)
                .into_par_iter()
                .map(|_| (self.parent_selection_op)(&population))
                .collect();

            println!("Making kids...");
            let children: Vec<Rated<Chromosome>> = parents.into_par_iter()
                .flat_map_iter(|(parent1, parent2)| {
                    let (child1, child2) = if Promile(thread_rng().gen_range(0..=1000)) <= self.params.crossover_probability {
                        (self.crossover_op)(parent1.value, parent2.value)
                    } else {
                        (parent1.value, parent2.value)
                    };

                    [child1, child2].into_iter()
                        .map(|mut child| {
                            (self.mutation_op)(&mut child, &self.params);
                            let rating = (self.fitness_function)(&child);
                            Rated { value: child, rating }
                        })
                })
                .collect();

            println!("Choosing next generation...");
            let next_generation: Vec<Rated<Chromosome>> = (0..self.params.population_size)
                .into_par_iter()
                .map(|_| (self.survivor_selection_op)(&children))
                .collect();
            population = next_generation;

            if i % LOG_FREQUENCY == 0 {
                let best_rating = population.iter().max()
                    .unwrap().rating;
                let iteration = Iteration { iteration: i, best_rating };
                println!("Iteration: {i}, best rating: {best_rating}");
                history.0.push_front(iteration);
            }
            if i % self.params.adjustment_rate == 0 {
                (self.adjust)(&mut self, &history);
            }
            i += 1;
        }

        let best_result = population.into_iter()
            .max().unwrap();
        let decoded = self.decoder.decode(&best_result.value);
        return decoded;
    }
}