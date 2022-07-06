use super::*;
use itertools::Itertools;

// Rating is inverse of number of class conflicts.
pub fn inverse_of_no_class_conflicts(solver: &Solution, genotype: &Chromosome) -> u32 {
    let decoded = solver.decode(genotype);

    let mut grouped_by_hour = decoded
        .into_iter()
        .map(|x| (x.lesson_hour, x))
        .into_group_map();

    grouped_by_hour.retain(|_, v| v.len() < 2);
    let mut conflicts: u32 = 0;
    for group in grouped_by_hour {
        conflicts += group.1.len() as u32;
    }
    let ratio = 1.0 / conflicts as f32;
    const PRECISION: f32 = 1000.0;

    (ratio * PRECISION) as u32
}