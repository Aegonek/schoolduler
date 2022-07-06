pub mod unsigned;
#[cfg(test)]
mod tests;

use std::mem;

pub trait Bits: Sized {
    fn bit_count(&self) -> usize {
        mem::size_of::<Self>() * 8
    }

    fn get_bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize, value: bool);

    fn modify_bit<F>(&mut self, index: usize, func: F) where F: FnOnce(&mut bool) {
        let mut bit = self.get_bit(index);
        func(&mut bit);
        self.set_bit(index, bit);
    }
}