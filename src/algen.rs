pub mod encoding;
pub mod history;
pub mod random;
pub mod solution;
pub mod config;

/// Expecting that we only mutate the hour.
/// Data necessary to:
/// 1. decode the lesson with Decoder that encoded that data
/// 2. efficiently rate fitness
#[derive(Debug, Clone, Copy, Hash)]
pub struct Gene {
    pub hour: u16,
    pub teacher: u16,
    pub student_group: u16
}

// Index of Gene in chromosome represents course for which we are assigning.
#[derive(Debug, Default, Clone, Hash)]
pub struct Chromosome(pub Vec<Gene>);
