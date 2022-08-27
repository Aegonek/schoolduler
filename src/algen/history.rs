use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Iteration {
    pub iteration: usize,
    pub best_rating: u32,
}

impl Display for Iteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Iteration: {} , best result: {}", self.iteration, self.best_rating))?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct History(/* used as stack */ pub VecDeque<Iteration>);

impl History {
    pub fn new() -> Self {
        History(VecDeque::new())
    }
}