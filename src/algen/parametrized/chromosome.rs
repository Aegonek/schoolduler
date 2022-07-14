pub trait IsChromosome: Clone + Sized + AsRef<[u8]> + Send + Sync /* dump raw data into sqlite */ {
    // Data necessary to select single gene from chromosome.
    type Index: Copy;
    type Indices: Iterator<Item = Self::Index>;

    fn indices(&self) -> Self::Indices;
}