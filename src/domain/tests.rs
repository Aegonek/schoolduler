use std::error::Error;

use super::*;
use crate::utils::testing::Case;
use tap::Tap;
use serde_json;

#[test]
fn deserialization_works() -> Result<(), Box<dyn Error>> {
    let raw = include_str!("../../data/example-courses.json");
    let _courses: Vec<Course> = serde_json::from_str(raw)?;
    return Ok(())
}

#[test]
fn correct_no_hours_in_week() {
    for Case {payload: course, expected} in correct_no_hours_in_week_cases() {
        assert!(course.subject.required_weekly_hours() == expected)
    }
}

fn correct_no_hours_in_week_cases() -> [Case<Course, u32>; 3] {
    let example = Course {
        subject: Subject {
            name: "Maths".into(),
            required_yearly_hours: 30
        },
        student_group: StudentGroup {
            year: 1,
            suffix: "A".into(),
        },
        teacher: Teacher {
            name: "John Smith".into(),
        }
    };
    [ 
        Case { payload: example.clone().tap_mut(|x| x.subject.required_yearly_hours = 30), expected: 1 }
        , Case { payload: example.clone().tap_mut(|x| x.subject.required_yearly_hours = 60), expected: 2 }
        , Case { payload: example.clone().tap_mut(|x| x.subject.required_yearly_hours = 120), expected: 3 }
    ]
}