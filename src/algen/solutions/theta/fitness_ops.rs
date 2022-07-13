use super::*;
use itertools::Itertools;

// Rating is inverse of number of class conflicts.
pub fn inverse_of_no_class_conflicts(solver: &Solution, chromosome: &Chromosome) -> u32 {
    // Best result - 0 conflicts
    // Worst result - every class has every lesson scheduled for the same hour...
    // and every teacher has every lesson scheduled for the same hour.

    let decoded = solver.decode(chromosome);

    todo!();
}

// TODO: test
fn overlapping_lessons_for_teacher(lessons: &[Class], max_possible_conflicts: &mut usize) -> usize {
    let mut conflicts: usize = 0;
    let mut working_copy = lessons.to_owned();
    working_copy.sort_by_key(|x| x.teacher.clone());
    let teacher_groups = working_copy.into_iter().group_by(|x| x.teacher.clone());
    for teacher_group in teacher_groups.into_iter() {
        let mut teacher_group: Vec<_> = teacher_group.1.collect();
        *max_possible_conflicts += teacher_group.len();
        teacher_group.sort_by_key(|x| x.lesson_hour);
        let hour_groups = teacher_group.into_iter().group_by(|x| x.lesson_hour.clone());
        for hour_group in hour_groups.into_iter() {
            let len = hour_group.1.count();
            if len > 1 { conflicts += len }
        }
    }

    conflicts
}