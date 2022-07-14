use crate::utils::units::Promile;
use super::algorithm::Algorithm;
use super::execution::History;

pub trait IsConfig<T: Algorithm>: Send + Sync {
    fn population_size(&self) -> usize;
    fn mutation_probability(&self) -> Promile;
    fn crossover_probability(&self) -> Promile;
    fn children_per_parent(&self) -> usize;

    fn adjust(&mut self, history: &History<T>);
}