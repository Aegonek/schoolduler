use std::collections::VecDeque;
use std::fmt::Display;

use crate::utils::rated::{Rated, Rating};

use super::Chromosome;

#[derive(Clone, Copy)]
pub struct Iteration {
    pub iteration: usize,
    pub best_rating: Rating,
}

impl Display for Iteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Iteration: {}, best result: {}",
            self.iteration, self.best_rating
        )?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Leaderboard {
    /* used as stack */
    pub iterations: VecDeque<Iteration>,
    pub max_teacher_overlaps: usize,
    pub max_group_overlaps: usize,
    pub winner: Option<Rated<Chromosome>>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Leaderboard {
            iterations: VecDeque::new(),
            max_teacher_overlaps: 0,
            max_group_overlaps: 0,
            winner: None,
        }
    }
}
