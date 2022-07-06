pub trait Genes: Clone + Sized {
    type Gene;
    fn genes(&mut self) -> &mut [Self::Gene];
}