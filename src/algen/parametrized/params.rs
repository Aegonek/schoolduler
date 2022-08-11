use crate::utils::ratio::Promile;

#[derive(Debug, PartialEq, Eq)]
pub struct Params {
    pub population_size: usize,
    pub mutation_probability: Promile,
    pub crossover_probability: Promile,
    pub children_per_parent: usize,
    /// Adjust config once per ADJUSTMENT_RATE iterations.
    pub adjustment_rate: usize
}