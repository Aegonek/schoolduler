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

macro_rules! impl_bit_traits {
    ($($typ:ty),+ $(,)?) => { 
        $(
            /// Least significant byte is on index 0, most significant byte on index bit_count() - 1
            impl Bits for $typ {
                fn bit_count(&self) -> usize {
                    mem::size_of::<$typ>() * 8
                }

                fn get_bit(&self, index: usize) -> bool {
                    if index >= self.bit_count() {
                        panic!("Trying to access index that is out of {} bounds! Index: {}", stringify!($typ), index);
                    }
                    let shift = Self::from(true) << index;
                    let masked = self & shift;
                    if masked != 0 {
                        true
                    } else { false }
                }

                fn set_bit(&mut self, index: usize, value: bool) {
                    if index >= self.bit_count() {
                        panic!("Trying to access index that is out of {} bounds! Index: {}", stringify!($typ), index);
                    }
                    let shift = Self::from(true) << index;
                    match value {
                        true => { *self = *self | shift; }
                        false => { *self = *self & !shift }
                    }
                }
            }
        )+
    };
}

impl_bit_traits!(u8, u16, u32, u64);