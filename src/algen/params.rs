use serde::Deserialize;
use serde::Serialize;

use crate::utils::ratio::Percent;
use crate::utils::ratio::Promile;
use std::ops::Range;

pub const LOG_FREQUENCY: usize = 10;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Params {
    pub population_size: usize,
    pub mutation_probability: Promile,
    pub crossover_probability: Promile,
    pub children_per_parent: usize,
    /// Adjust config once per ADJUSTMENT_RATE iterations.
    pub adjustment_rate: usize,
    // Point of below ops for now: serialize info about operations we used in benchmarks.
    pub fitness_function: FitnessFunction,
    pub parent_selection_op: ParentSelectionOp,
    pub crossover_op: CrossoverOp,
    pub mutation_op: MutationOp,
    pub survivor_selection_op: SurvivalSelectionOp,
    pub adjust_strategy: AdjustStrategy,
    pub termination_condition: TerminationCondition,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            population_size: 1000,
            mutation_probability: Percent(7).into(),
            crossover_probability: Percent(80).into(),
            children_per_parent: 1,
            adjustment_rate: 10,
            fitness_function: FitnessFunction::InverseOfNoClassConflicts,
            parent_selection_op: ParentSelectionOp::RouletteSelection,
            crossover_op: CrossoverOp::OnePointCrossover,
            mutation_op: MutationOp::InvertBitMutation,
            survivor_selection_op: SurvivalSelectionOp::RouletteSelection,
            adjust_strategy: AdjustStrategy::NoAdjustment,
            termination_condition: TerminationCondition::AfterNoIterations(1000),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FitnessFunction {
    InverseOfNoClassConflicts,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrossoverOp {
    OnePointCrossover,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationOp {
    InvertBitMutation,
    // TODO: fix + -
    CreepMutation { creep_range: Range<u8> },
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdjustStrategy {
    NoAdjustment,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentSelectionOp {
    RouletteSelection,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurvivalSelectionOp {
    RouletteSelection,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationCondition {
    AfterNoIterations(usize),
}
