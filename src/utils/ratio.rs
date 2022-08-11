use derive_more::{Add, Sub};


// Dumb wrappers written so we don't forget units.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Add, Sub)]
pub struct Promile(pub u32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Add, Sub)]
pub struct Percent(pub u32);

impl From<Percent> for Promile {
    fn from(percent: Percent) -> Self {
        Promile(percent.0 * 10)
    }
}