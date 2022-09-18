use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

// TODO: make it so the unit is serialized.

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Probability {
    Promile(u32),
    Percent(u32)
}

use Probability::*;

impl Probability {
    pub fn percent(&self) -> u32 {
        match self {
            Promile(x) => x / 10,
            Percent(x) => *x
        }
    }

    pub fn promile(&self) -> u32 {
        match self {
            Promile(x) => *x,
            Percent(x) => x * 10
        }
    }
}

impl Add<Probability> for Probability {
    type Output = Self;

    fn add(self, rhs: Probability) -> Self::Output {
        let lhs = self.promile();
        let rhs = rhs.promile();
        Promile(lhs + rhs)
    }
}

impl Sub<Probability> for Probability {
    type Output = Self;

    fn sub(self, rhs: Probability) -> Self::Output {
        let lhs = self.promile();
        let rhs = rhs.promile();
        Promile(lhs - rhs)
    }
}