use super::*;
use itertools::Itertools;
use num::traits::Pow;

// Rating is inverse of number of class conflicts.
pub fn inverse_of_no_class_conflicts(solver: &Solution, chromosome: &Chromosome) -> u32 {
    let decoded = solver.decode(chromosome);

    let mut grouped_by_hour = decoded
        .into_iter()
        .map(|x| (x.lesson_hour, x))
        .into_group_map();

    grouped_by_hour.retain(|_, v| v.len() > 1);
    let mut conflicts: u32 = 0;
    for group in grouped_by_hour {
        conflicts += group.1.len() as u32;
    }
    let ratio = 1.0 / conflicts as f64;
    const DIGITS: u8 = 9;
    let precision: f64 = (10.0 as f64).pow(DIGITS);

    (ratio * precision).round() as u32
}