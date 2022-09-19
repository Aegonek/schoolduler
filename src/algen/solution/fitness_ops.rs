use std::cmp;

use itertools::Itertools;

use super::*;
use crate::{utils, algen::Gene};

#[cfg(test)]
mod tests;

// Rating is inverse of number of class conflicts.
pub fn inverse_of_no_class_conflicts(
    chromosome: &Chromosome
) -> Rating {
    let lessons = &chromosome.0;

    let teacher_overlaps = teacher_overlaps(&lessons);
    let mut teacher_score = 2_u32.pow(teacher_overlaps);
    teacher_score = cmp::min(teacher_score, u32::MAX);
    let teacher_score = utils::num::map_range(teacher_score, 0..=u32::MAX, 1..=0);

    let group_overlaps = group_overlaps(&lessons);
    let mut group_score = 2_u32.pow(group_overlaps);
    group_score = 2_u32.pow(group_score);
    let group_score = utils::num::map_range(group_score, 0..=u32::MAX, 1..=0);

    Rating::MAX * ((teacher_score + group_score) / 2.0)
}

// If one student group has 3 classes scheduled for same hour, it counts as 2 conflicts.
// Number of conflicts at hour `x` = number of lessons scheduled for `x` - 1
fn teacher_overlaps(lessons: &[Gene]) -> u32 {
    let mut conflicts: u32 = 0;

    let teachers = lessons
        .into_iter()
        .into_group_map_by(|cls| cls.teacher.clone());

    for teacher in teachers.values() {
        let hours = teacher.into_iter().into_group_map_by(|cls| cls.hour);

        for duplicates in hours.values().filter(|xs| xs.len() > 1) {
            conflicts += (duplicates.len() - 1) as u32
        }
    }

    conflicts
}

fn group_overlaps(lessons: &[Gene]) -> u32 {
    let mut conflicts: u32 = 0;

    let groups = lessons
        .into_iter()
        .into_group_map_by(|cls| cls.student_group.clone());

    for group in groups.values() {
        let hours = group.into_iter().into_group_map_by(|cls| cls.hour);

        for duplicates in hours.values().filter(|xs| xs.len() > 1) {
            conflicts += (duplicates.len() - 1) as u32
        }
    }

    conflicts
}
