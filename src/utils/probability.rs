use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Probability {
    Promile(u32),
    Percent(u32)
}

use Probability::*;

impl Probability {
    pub const fn percent(&self) -> u32 {
        match self {
            Promile(x) => *x / 10,
            Percent(x) => *x
        }
    }

    pub const fn promiles(&self) -> u32 {
        match self {
            Promile(x) => *x,
            Percent(x) => *x * 10
        }
    }
}

impl Add<Probability> for Probability {
    type Output = Self;

    fn add(self, rhs: Probability) -> Self::Output {
        let lhs = self.promiles();
        let rhs = rhs.promiles();
        Promile(lhs + rhs)
    }
}

impl Sub<Probability> for Probability {
    type Output = Self;

    fn sub(self, rhs: Probability) -> Self::Output {
        let lhs = self.promiles();
        let rhs = rhs.promiles();
        Promile(lhs - rhs)
    }
}

impl PartialOrd for Probability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(&self.promiles(), &other.promiles()))
    }
}

impl Ord for Probability {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Self::partial_cmp(self, other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn expected_result_conversion() {
        use super::Probability::*;
        
        let percent = Percent(7);
        assert_eq!(percent.promiles(), 70);
        let promile = Promile(20);
        assert_eq!(promile.percent(), 2);
    }
}