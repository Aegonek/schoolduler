use crate::utils::rated::Rated;
use super::algorithm::Algorithm;

#[derive(Clone)]
pub struct Iteration<T>
where
    T: Algorithm,
{
    pub iteration: usize,
    pub best_result: Rated<T::Chromosome>,
}

#[derive(Clone)]
pub struct ExecutionContext<T>
where
    T: Algorithm,
{
    pub iteration_count: usize,
    pub history: Vec<Iteration<T>>,
}

impl<T: Algorithm> Default for ExecutionContext<T> {
    fn default() -> Self {
        Self {
            iteration_count: Default::default(),
            history: Default::default(),
        }
    }
}