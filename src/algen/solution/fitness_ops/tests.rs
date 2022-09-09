// Co powinienem sprawdzić?
// 1. Liczba konfliktów jest faktycznie taka jak oczekiwana.

use once_cell::sync::Lazy;

use super::*;

static SCHEDULE: Lazy<Schedule> = Lazy::new(|| {
    const RAW: &'static str = include_str!(r"..\..\..\..\input\random-schedule.json");
    serde_json::from_str(RAW).expect("Deserialization of random schedule failed.")
});

fn schedule() -> &'static [Class] { &SCHEDULE }

#[test]
fn expected_no_overlapping_lessons_for_teacher() {
    let schedule = schedule();
    let overlaps = teacher_overlaps(&schedule);
    assert_eq!(overlaps, 2);
}

#[test]
fn expected_no_overlapping_lessons_for_student_group() {
    let schedule = schedule();
    let overlaps = group_overlaps(&schedule);
    assert_eq!(overlaps, 1)
}