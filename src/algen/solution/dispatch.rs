use std::borrow::Borrow;

use super::crossover_ops;
use super::fitness_ops;
use super::mutation_ops;
use super::*;

impl Solution {
    pub fn rate(&mut self, chrom: &Chromosome) -> Rating {
        use FitnessFunction::*;

        match self.params.fitness_function {
            InverseOfNoClassConflicts => fitness_ops::inverse_of_no_class_conflicts(
                chrom,
                &mut self.leaderboard,
            ),
        }
    }

    pub fn rated<T: Borrow<Chromosome>>(&mut self, chrom: T) -> Rated<T> {
        let rating = self.rate(chrom.borrow());
        Rated {
            value: chrom,
            rating,
        }
    }

    pub fn mutate(&self, chrom: &mut Chromosome) {
        use MutationOp::*;

        match &self.params.mutation_op {
            InvertBitMutation => mutation_ops::invert_bit_mutation(chrom, &self.params),
            CreepMutation { creep_range } => {
                // mutation_ops::creep_mutation(chrom, &self.params, &creep_range)
                todo!()
            }
        };
    }

    // Assuming always 2 parents and always 2 children.
    pub fn crossover(&self, x: Chromosome, y: Chromosome) -> (Chromosome, Chromosome) {
        use CrossoverOp::*;

        match self.params.crossover_op {
            OnePointCrossover => crossover_ops::one_point_crossover(x, y),
        }
    }

    pub fn select_parents<'a>(
        &self,
        population: &'a [Rated<Chromosome>],
    ) -> (&'a Rated<Chromosome>, &'a Rated<Chromosome>) {
        use ParentSelectionOp::*;

        match self.params.parent_selection_op {
            RouletteSelection => {
                let x = select_ops::roulette_selection(population);
                let y = select_ops::roulette_selection(population);
                (x, y)
            }
        }
    }

    pub fn select_survivor<'a>(
        &self,
        population: &'a [Rated<Chromosome>],
    ) -> &'a Rated<Chromosome> {
        use ParentSelectionOp::*;

        match self.params.parent_selection_op {
            RouletteSelection => select_ops::roulette_selection(population),
        }
    }

    pub fn adjust(&mut self) {
        use AdjustStrategy::*;

        match self.params.adjust_strategy {
            NoAdjustment => (),
        }
    }

    pub fn should_terminate(&self) -> bool {
        use TerminationCondition::*;

        match self.leaderboard.winner.as_ref() {
            Some(&Rated { rating, .. }) if rating == Rating::MAX => return true,
            _ => ()
        }

        match self.params.termination_condition {
            AfterNoIterations(n) => {
                self.leaderboard.iterations.front().map(|x| x.iteration).unwrap_or(0) >= n
            }
        }
    }
}
