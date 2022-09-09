pub mod encoding;
pub mod history;
pub mod params;
pub mod random;
pub mod solution;

use bitvec::vec::BitVec;
use derive_more::{AsMut, AsRef};

// Index of u8 in chromosome represents course for which we are assigning.
// Value of u8 in chromosome represents lesson hour assigned to it.
#[derive(Debug, Default, Clone, AsRef, AsMut)]
pub struct Chromosome(pub BitVec<u8>);
