use super::*;

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
            mutation_probability: Percent(7).into(),
            crossover_probability: Percent(80).into(),
            children_per_parent: 1,
            mutation_op: MutationOp::InvertBitMutation,
            termination_condition: TerminationCondition::AfterNoIterations(100_000),
        }
    }
}

impl IsConfig<Solution> for Config {
    fn population_size(&self) -> usize { self.population_size }

    fn mutation_probability(&self) -> Promile { self.mutation_probability }

    fn crossover_probability(&self) -> Promile { self.crossover_probability }

    fn children_per_parent(&self) -> usize { self.children_per_parent }

    // TODO: consider adjusting probability based on history
    fn adjust(&mut self, _history: &History<Solution>) { }
}