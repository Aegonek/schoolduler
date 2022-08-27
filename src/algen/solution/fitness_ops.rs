use itertools::Itertools;

use super::*;
use crate::utils::num;

// TODO: test
// TODO: benchmark, possibly optimize?
// Rating is inverse of number of class conflicts.
pub fn inverse_of_no_class_conflicts(chromosome: &Chromosome, decoder: &Decoder) -> Rating {
    // Best result - 0 conflicts
    // Worst result - every class has every lesson scheduled for the same hour...
    // and every teacher has every lesson scheduled for the same hour.

    let mut max_possible_conflicts: usize = 0;
    // This part is heavy lol. 
    // TODO: optimize
    let lessons = decoder.decode(chromosome);
    let teacher_conflicts = overlapping_lessons_for_teacher(&lessons, &mut max_possible_conflicts);
    let class_conflicts = overlapping_lessons_for_class(&lessons, &mut max_possible_conflicts);
    let ratio = (teacher_conflicts + class_conflicts) as f64 / max_possible_conflicts as f64;
    let rating = num::map_range(ratio, (0.0)..=(1.0), (1.0)..=(0.0));

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