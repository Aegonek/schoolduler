use crate::utils::indexed::{Len, GetIndexMut};

pub trait Genotype: Clone + Sized {
    type Gene<'a> where Self: 'a;
    type Genes<'a>: GetIndexMut + Len where Self: 'a;

    fn genes(&mut self) -> &mut Self::Genes<'_>;
}