pub mod random;
pub mod params;
pub mod history;
pub mod encoding;
pub mod algorithm;

use bitvec::vec::BitVec;
use derive_more::{AsRef, AsMut};

#[derive(Debug, Default, Clone, AsRef, AsMut)]
pub struct Chromosome(pub BitVec<u8>);