use crate::utils::ratio::Percent;
use crate::utils::ratio::Promile;
use std::ops::Range;

pub const LOG_FREQUENCY: usize = 10;

#[derive(Debug, PartialEq, Eq)]
pub struct Params {
    pub population_size: usize,
    pub mutation_probability: Promile,
    pub crossover_probability: Promile,
    pub children_per_parent: usize,
    /// Adjust config once per ADJUSTMENT_RATE iterations.
    pub adjustment_rate: usize
}

impl Default for Params {
    fn default() -> Self {
        Params {
            population_size: 1000,
            mutation_probability: Percent(7).into(),
            crossover_probability: Percent(80).into(),
            children_per_parent: 1,
            adjustment_rate: 10
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MutationOp {
    InvertBitMutation,
    CreepMutation { creep_range: Range<u8> }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdjustStrategy {
    NoAdjustment
}

#[derive(Debug, PartialEq, Eq)]
pub enum FitnessFunction {
    InverseOfNoClassConflicts
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrossoverOp {
    OnePointCrossover
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParentSelectionOp {
    RouletteSelection
}

#[derive(Debug, PartialEq, Eq)]
pub enum SurvivalSelectionOp {
    RouletteSelection
}

#[derive(Debug, PartialEq, Eq)]
pub enum TerminationCondition {
    AfterNoIterations(usize),
}