use super::*;
use itertools::Itertools;

// Rating is inverse of number of class conflicts.
pub fn inverse_of_no_class_conflicts(solver: &Solution, chromosome: &Chromosome) -> u32 {
    // Best result - 0 conflicts
    // Worst result - every class has every lesson scheduled for the same hour...
    // and every teacher has every lesson scheduled for the same hour.

    let mut max_possible_conflicts: usize = 0;
    let lessons = solver.decode(chromosome);
    let teacher_conflicts = overlapping_lessons_for_teacher(&lessons, &mut max_possible_conflicts);
    let class_conflicts = overlapping_lessons_for_class(&lessons, &mut max_possible_conflicts);
    let ratio = (teacher_conflicts + class_conflicts) as f64 / max_possible_conflicts as f64;
    let rating = map_number_range(ratio, Range { start: 0.0, end: 1.0 }, Range { start: 1.0, end: 0.0 });

    let rating = rating.powf(1.7);
    // 9 first digits
    let rating = (rating * 100_000.0) as u32;
    rating
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

fn overlapping_lessons_for_class(lessons: &[Class], max_possible_conflicts: &mut usize) -> usize {
    let mut conflicts: usize = 0;
    let mut working_copy = lessons.to_owned();
    working_copy.sort_by_key(|x| x.teacher.clone());
    let student_groups = working_copy.into_iter().group_by(|x| x.student_group.clone());
    for student_group in student_groups.into_iter() {
        let mut student_group: Vec<_> = student_group.1.collect();
        *max_possible_conflicts += student_group.len();
        student_group.sort_by_key(|x| x.lesson_hour);
        let hour_groups = student_group.into_iter().group_by(|x| x.lesson_hour.clone());
        for hour_group in hour_groups.into_iter() {
            let len = hour_group.1.count();
            if len > 1 { conflicts += len }
        }
    }

    conflicts
}

// TEST: 0.7; 0, 1; 1, 0
// TODO: write test
fn map_number_range(nmb: f64, old: Range<f64>, new: Range<f64>) -> f64 {
    // (1 - 0) / (0 - 1)
    // 1 / -1
    // -1 
    // ratio between ranges
    let ratio = (new.end - new.start) / (old.end - old.start);
    // (0.7 - 0) * ratio + 1
    // 0.7 * -1 + 1
    // 0.3
    let new = (nmb - old.start) * ratio + new.start;
    new
}