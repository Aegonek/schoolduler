use std::fmt::Display;

pub trait IsChromosome: Clone + Sized + Display + AsRef<[u8]> /* dump raw data into sqlite */ {
    // Data necessary to select single gene from chromosome.
    type Index: Copy;
    type Indices: Iterator<Item = Self::Index>;

    fn indices(&self) -> Self::Indices;
}