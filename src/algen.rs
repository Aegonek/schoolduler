pub mod encoding;
pub mod history;
pub mod params;
pub mod random;
pub mod solution;
use derive_more::{AsMut, AsRef};

/// Expecting that we only mutate the hour.
/// Data necessary to:
/// 1. decode the lesson with Decoder that encoded that data
/// 2. efficiently rate fitness
#[derive(Debug, Clone, Copy)]
pub struct Gene {
    pub hour: u8,
    pub teacher: u8,
    pub student_group: u8
}

// Index of Gene in chromosome represents course for which we are assigning.
#[derive(Debug, Default, Clone, AsRef, AsMut)]
pub struct Chromosome(pub Vec<Gene>);
