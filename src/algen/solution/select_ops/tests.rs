use crate::utils::{tests::chromosome, rating::{Rated, Rating}};

pub static mut RANDOM: u64 = 0;

#[test]
fn expected_result_roulette_selection() {
    let population = [
        Rated::new(chromosome!(0, 24, 67, 89), Rating::new(25)),
        // chromosome!(50, 254, 23, 45),
        // chromosome!(7, 8, 14, 6),
        // chromosome!(2, 3, 6, 98),
    ];
}