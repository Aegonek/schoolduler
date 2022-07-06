use derive_more::{Add, Sub};

// Dumb wrappers written so we don't forget units.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Add, Sub)]
pub struct Promile(pub u16);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Add, Sub)]
pub struct Percent(pub u8);