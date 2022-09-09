use derive_more::{Add, Sub};
use serde::{Deserialize, Serialize};

// Dumb wrappers written so we don't forget units.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Add, Sub, Serialize, Deserialize)]
pub struct Promile(pub u32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Add, Sub, Serialize, Deserialize)]
pub struct Percent(pub u32);

impl From<Percent> for Promile {
    fn from(percent: Percent) -> Self {
        Promile(percent.0 * 10)
    }
}
