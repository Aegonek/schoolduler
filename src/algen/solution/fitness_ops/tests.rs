// Co powinienem sprawdzić?
// 1. Liczba konfliktów jest faktycznie taka jak oczekiwana.

use time::{Duration, Time, Weekday};

use crate::utils::num::assert_approx_eq;

use super::*;

#[test]
fn expected_rating() {
    let sched1: Vec<_> = GenClasses::new()
        .dup_teacher(4)
        .take(50)
        .collect();
    let mut dec1 = Decoder::new();
    let sched1 = dec1.encode(&sched1);

    let sched2: Vec<_> = GenClasses::new()
        .dup_teacher(6)
        .dup_group(5)
        .take(50)
        .collect();
    let mut dec2 = Decoder::new();
    let sched2 = dec2.encode(&sched2);

    let sched3: Vec<_> = GenClasses::new()
        .take(50)
        .collect();
    let mut dec3 = Decoder::new();
    let sched3 = dec3.encode(&sched3);

    let sched4: Vec<_> = GenClasses::new()
        .dup_teacher(4) // 3 / 5 -> 0.4 * DIGITS -> 40_000
        .dup_group(2) // 1 / 4 -> 0.75 * DIGITS -> 75_000
        .take(50)
        .collect(); // Rating - avg(40_000, 75_000) -> 57_500
    let mut dec4 = Decoder::new();
    let sched4 = dec4.encode(&sched4);

    let mut leaderboard = Leaderboard::new();

    // We expect crappy result like this for first iterations, it should quickly stabilize.
    // How to test if it stabilizes?
    assert_eq!(inverse_of_no_class_conflicts(&sched1, &dec1, &mut leaderboard), 50_000);
    assert_eq!(inverse_of_no_class_conflicts(&sched2, &dec2, &mut leaderboard), 0);
    assert_eq!(inverse_of_no_class_conflicts(&sched3, &dec3, &mut leaderboard), 100_000);
    assert_approx_eq!(inverse_of_no_class_conflicts(&sched4, &dec4, &mut leaderboard), 57_500, 10);
}

#[test]
fn expected_number_of_overlaps() {
    let mut schedule: Vec<Class> = Vec::new();
    GenClasses::new()
        .dup_teacher(8)
        .poll(25, &mut schedule)
        .dup_group(7)
        .poll(25, &mut schedule);
    let teacher_overlaps = teacher_overlaps(&schedule);
    assert_eq!(teacher_overlaps, 7);
    let group_overlaps = group_overlaps(&schedule);
    assert_eq!(group_overlaps, 6);
}

#[test]
fn expected_number_of_teacher_overlaps() {
    let schedule: Vec<_> = GenClasses::new().dup_teacher(4).take(50).collect();
    let overlaps = teacher_overlaps(&schedule);
    assert_eq!(overlaps, 3);
}

#[test]
fn expected_number_of_group_overlaps() {
    let schedule: Vec<_> = GenClasses::new().dup_group(2).take(50).collect();
    let overlaps = group_overlaps(&schedule);
    assert_eq!(overlaps, 1)
}

//---- Utilities

struct GenClasses {
    group_seed: usize,
    teacher_seed: usize,
    dup_group: Option<usize>,
    dup_teachers: Option<usize>,
}

impl GenClasses {
    pub fn new() -> Self {
        GenClasses {
            group_seed: 0,
            teacher_seed: 0,
            dup_group: None,
            dup_teachers: None,
        }
    }

    // Poll `n` next iterations, collecting their results into `container`. Return an iterator after polling.
    pub fn poll(&mut self, n: usize, container: &mut Vec<Class>) -> &mut Self {
        for _ in 0..n {
            let cls = self.next().unwrap();
            container.push(cls);
        }
        self
    }

    // Request that `conflicts` next iterations yield the same teacher at the same hour.
    pub fn dup_teacher(&mut self, conflicts: usize) -> &mut Self {
        self.dup_teachers = Some(conflicts - 1);
        self
    }

    pub fn dup_group(&mut self, conflicts: usize) -> &mut Self {
        self.dup_group = Some(conflicts - 1);
        self
    }
}

// Yield classes with fixed hour and distinct (though nonsense) student_group and teacher.
// You can request teacher to be duplicated in next iterations through impl.
impl Iterator for GenClasses {
    type Item = Class;

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(Class {
            subject: Subject {
                name: "Math".to_owned(),
                required_yearly_hours: 0,
            },
            student_group: StudentGroup {
                year: self.group_seed as u16,
                suffix: self.group_seed.to_string(),
            },
            teacher: Teacher {
                name: self.teacher_seed.to_string(),
            },
            lesson_hour: LessonHour {
                weekday: Weekday::Monday,
                time: Time::from_hms(8, 0, 0).unwrap(),
                duration: Duration::minutes(45),
            },
        });

        if let Some(i) = self.dup_group {
            self.dup_group = if i - 1 > 0 { Some(i - 1) } else { None }
        } else {
            self.group_seed += 1;
        }

        if let Some(i) = self.dup_teachers {
            self.dup_teachers = if i - 1 > 0 { Some(i - 1) } else { None }
        } else {
            self.teacher_seed += 1;
        }

        res
    }
}
