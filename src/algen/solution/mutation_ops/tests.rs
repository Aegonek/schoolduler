use crate::utils::tests::*;

pub static mut CREEP: i16 = 0;

#[test]
fn expected_results_invert_bit() {
    let mut chromosome = chromosome!(5, 24, 127, 50);
    invert_bit_mutation(&mut chromosome);
    let hours = hours(chromosome);
    assert_eq!(hours, vec![130, 159, 248, 181]);
}

#[test]
fn expected_results_creep_mutation() {
    {
        let mut chromosome = chromosome!(5, 24, 127, 50, u16::MIN, u16::MAX);
        unsafe { CREEP = 20 }
        creep_mutation(&mut chromosome);
        let hours = hours(chromosome);
        assert_eq!(hours, vec![25, 44, 147, 70, u16::MIN + 20, u16::MAX])
    }
    {
        let mut chromosome = chromosome!(5, 24, 127, 50, u16::MIN, u16::MAX);
        unsafe { CREEP = -20 }
        creep_mutation(&mut chromosome);
        let hours = hours(chromosome);
        assert_eq!(hours, vec![0, 4, 107, 30, u16::MIN, u16::MAX - 20])
    }
}

use super::{invert_bit_mutation, creep_mutation};