use num::Integer;
use time::util::weeks_in_year;
use time::{OffsetDateTime, Time, Weekday};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Teacher {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StudentGroup {
    pub year: u16,
    pub sufix: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subject {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LessonBlock {
    pub subject: Subject,
    pub student_group: StudentGroup,
    pub teacher: Teacher,
    pub required_yearly_hours: u32,
}

impl LessonBlock {
    pub fn required_weekly_hours(&self) -> u32 {
        let current_year = OffsetDateTime::now_utc().year();
        self.required_yearly_hours
            .div_ceil(&u32::from(weeks_in_year(current_year)))
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct RepeatingLessonHour {
    pub weekday: Weekday,
    pub time: Time,
    pub duration: u32, // in minutes
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Class {
    pub subject: Subject,
    pub student_group: StudentGroup,
    pub teacher: Teacher,
    pub lesson_hour: RepeatingLessonHour,
}
