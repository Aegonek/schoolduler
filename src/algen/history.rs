use std::collections::VecDeque;
use std::fmt::Display;

use crate::utils::rated::Rating;

#[derive(Clone, Copy)]
pub struct Iteration {
    pub iteration: usize,
    pub best_rating: Rating,
}

impl Display for Iteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Iteration: {} , best result: {}", self.iteration, self.best_rating))?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Leaderboard {
    /* used as stack */ 
    pub iterations: VecDeque<Iteration>,
    pub max_teacher_overlaps: usize,
    pub max_group_overlaps: usize,
    pub worst_result: usize,
    pub best_result: usize
}

impl Leaderboard {
    pub fn new() -> Self {
        Leaderboard {
            iterations: VecDeque::new(), 
            max_teacher_overlaps: 0, 
            max_group_overlaps: 0, 
            worst_result: 0, 
            best_result: 0,
         }
    }
}

impl Default for Leaderboard {
    fn default() -> Self { Self::new() }
}