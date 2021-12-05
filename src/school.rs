mod lessons; // seperating definitions for easier browsing, I'm reexporting them together

pub use lessons::*;
use time::{Duration, Time, Weekday};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Teacher {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct StudentGroup {
    pub year: u16,
    pub sufix: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Subject {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct RepeatingLessonHour {
    pub weekday: Weekday,
    pub time: Time,
    pub duration: Duration, // in minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_no_hours_in_week() {
        let block = LessonBlock {
            subject: Subject {
                name: "Język polski".into(),
            },
            student_group: StudentGroup {
                year: 1,
                sufix: "F".into(),
            },
            teacher: Teacher {
                first_name: "Adam".into(),
                last_name: "Tuszyński".into(),
            },
            required_yearly_hours: 60,
        };
        let weekly_hours = block.required_weekly_hours();
        assert!(weekly_hours == 2)
    }
}
