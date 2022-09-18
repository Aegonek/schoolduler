use serde::{Deserialize, Serialize};

use crate::utils::probability::Probability::{self, Percent};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub population_size: usize,
    pub mutation_probability: Probability,
    pub crossover_probability: Probability,
    pub children_per_parent: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            population_size: 50,
            mutation_probability: Percent(10),
            crossover_probability: Percent(95),
            children_per_parent: 1,
        }
    }
}

pub const CONFIG: Config = include!(concat!(env!("OUT_DIR"), "/config.generated.rs"));
