use itertools::Itertools;

use super::*;
use crate::{utils::num, algen::Gene};

#[cfg(test)]
mod tests;

// Rating is inverse of number of class conflicts.
pub fn inverse_of_no_class_conflicts(
    chromosome: &Chromosome,
    leaderboard: &mut Leaderboard,
) -> Rating {
    let lessons = &chromosome.0;

    let teacher_overlaps = teacher_overlaps(&lessons);
    if teacher_overlaps > leaderboard.max_teacher_overlaps {
        leaderboard.max_teacher_overlaps = teacher_overlaps;
    }
    let teacher_score = match leaderboard.max_teacher_overlaps {
        0 => 1.0,
        max => num::map_range(teacher_overlaps as f64, 0.0..=(max as f64), 1.0..=0.0),
    };

    let group_overlaps = group_overlaps(&lessons);
    if group_overlaps > leaderboard.max_group_overlaps {
        leaderboard.max_group_overlaps = group_overlaps;
    }
    let group_score = match leaderboard.max_group_overlaps {
        0 => 1.0,
        max => num::map_range(group_overlaps as f64, 0.0..=(max as f64), 1.0..=0.0),
    };

    Rating::MAX * ((teacher_score + group_score) / 2.0)
}

// If one student group has 3 classes scheduled for same hour, it counts as 2 conflicts.
// Number of conflicts at hour `x` = number of lessons scheduled for `x` - 1
fn teacher_overlaps(lessons: &[Gene]) -> usize {
    let mut conflicts: usize = 0;

    let teachers = lessons
        .into_iter()
        .into_group_map_by(|cls| cls.teacher.clone());

    for teacher in teachers.values() {
        let hours = teacher.into_iter().into_group_map_by(|cls| cls.hour);

        for duplicates in hours.values().filter(|xs| xs.len() > 1) {
            conflicts += duplicates.len() - 1
        }
    }

    conflicts
}

fn group_overlaps(lessons: &[Gene]) -> usize {
    let mut conflicts: usize = 0;

    let groups = lessons
        .into_iter()
        .into_group_map_by(|cls| cls.student_group.clone());

    for group in groups.values() {
        let hours = group.into_iter().into_group_map_by(|cls| cls.hour);

        for duplicates in hours.values().filter(|xs| xs.len() > 1) {
            conflicts += duplicates.len() - 1
        }
    }

    conflicts
}
