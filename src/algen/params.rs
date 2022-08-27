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

impl Default for MutationOp {
    fn default() -> Self {
        Self::InvertBitMutation
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdjustStrategy {
    NoAdjustment
}

impl Default for AdjustStrategy {
    fn default() -> Self {
        Self::NoAdjustment
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FitnessFunction {
    InverseOfNoClassConflicts
}

impl Default for FitnessFunction {
    fn default() -> Self {
        Self::InverseOfNoClassConflicts
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrossoverOp {
    OnePointCrossover
}

impl Default for CrossoverOp {
    fn default() -> Self {
        Self::OnePointCrossover
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParentSelectionOp {
    RouletteSelection
}

impl Default for ParentSelectionOp {
    fn default() -> Self {
        Self::RouletteSelection
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum SurvivalSelectionOp {
    RouletteSelection
}

impl Default for SurvivalSelectionOp {
    fn default() -> Self {
        Self::RouletteSelection
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TerminationCondition {
    AfterNoIterations(usize),
}

impl Default for TerminationCondition {
    fn default() -> Self {
        Self::AfterNoIterations(10000)
    }
}