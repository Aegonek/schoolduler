use crate::utils::{tests::chromosome, rating::{Rated, Rating}};

use super::roulette_selection;

pub static mut RANDOM: u64 = 0;

#[test]
fn expected_result_roulette_selection() {
    let population = [
        Rated::new(chromosome!(0, 24, 67, 89), Rating::new(25)),
        Rated::new(chromosome!(50, 254, 23, 45), Rating::new(75)),
        Rated::new(chromosome!(7, 8, 14, 6), Rating::new(40)),
        Rated::new(chromosome!(2, 3, 6, 98), Rating::new(60)),
    ];
    unsafe { RANDOM = 130 };
    let chosen = roulette_selection(&population);
    assert_eq!(chosen, &Rated::new(chromosome!(7, 8, 14, 6), Rating::new(40)));
}