use crate::utils::tests::*;

use super::one_point_crossover;

pub static mut CROSSOVER_AXIS: usize = 0;

#[test]
fn expected_results_one_point_crossover() {
    let parent1 = chromosome!(56, 20, 40, 38, 75, 52);
    let parent2 = chromosome!(32, 41, 79, 22, 69, 420);
    unsafe { CROSSOVER_AXIS = 3 }
    let (child1, child2) = one_point_crossover(parent1, parent2);

    assert_eq!(hours(child1), hours(chromosome!(56, 20, 40, 22, 69, 420)));
    assert_eq!(hours(child2), hours(chromosome!(32, 41, 79, 38, 75, 52)));
}