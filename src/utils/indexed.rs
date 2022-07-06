use bitvec::{prelude::*, ptr::Mut};

pub trait Len {
    fn len(&self) -> usize;
}

impl<T: BitStore, U: BitOrder> Len for BitVec<T, U> {
    fn len(&self) -> usize {
        self.len()
    }
}

pub trait GetIndexMut {
    type Output<'a> where Self: 'a;

    fn get_index_mut(&mut self, index: usize) -> Self::Output<'_>;
}

impl<T: BitStore, U: BitOrder> GetIndexMut for BitVec<T, U> {
    type Output<'a> = BitRef<'a, Mut, T, U>;

    fn get_index_mut(&mut self, index: usize) -> Self::Output<'_> {
        self.get_mut(index).unwrap()
    }
}