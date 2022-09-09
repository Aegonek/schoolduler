use std::error::Error;

use super::*;
use crate::utils::tests::Case;
use serde_json;

#[test]
fn deserialization_works() -> Result<(), Box<dyn Error>> {
    let raw = include_str!("../../input/example-courses.json");
    let _courses: Vec<Course> = serde_json::from_str(raw)?;
    return Ok(());
}

#[test]
fn correct_no_hours_in_week() {
    for Case {
        payload: course,
        expected,
    } in cases_correct_no_hours_in_week()
    {
        assert!(course.subject.required_weekly_hours() == expected)
    }
}

fn cases_correct_no_hours_in_week() -> [Case<Course, u32>; 3] {
    let example = Course {
        subject: Subject {
            name: "Maths".into(),
            required_yearly_hours: 30,
        },
        student_group: StudentGroup {
            year: 1,
            suffix: "A".into(),
        },
        teacher: Teacher {
            name: "John Smith".into(),
        },
    };
    [
        {
            let mut payload = example.clone();
            payload.subject.required_yearly_hours = 30;
            Case {
                payload,
                expected: 1,
            }
        },
        {
            let mut payload = example.clone();
            payload.subject.required_yearly_hours = 60;
            Case {
                payload,
                expected: 2,
            }
        },
        {
            let mut payload = example.clone();
            payload.subject.required_yearly_hours = 120;
            Case {
                payload,
                expected: 3,
            }
        },
    ]
}
