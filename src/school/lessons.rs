use super::*;
use derive_more::{From, Into};
use num::Integer;
use time::util::weeks_in_year;
use time::{OffsetDateTime};

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

    pub fn schedule_for(&self, hour: RepeatingLessonHour) -> Class {
        Class {
            subject: self.subject.clone(),
            student_group: self.student_group.clone(),
            teacher: self.teacher.clone(),
            lesson_hour: hour,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Requirements {
    pub lessons: Vec<LessonBlock>,
    pub open_hours: Vec<RepeatingLessonHour>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LessonInfo {
    pub subject: Subject,
    pub student_group: StudentGroup,
    pub teacher: Teacher,
}

impl LessonInfo {
    pub fn schedule_for(&self, hour: RepeatingLessonHour) -> Class {
        Class {
            subject: self.subject.clone(),
            student_group: self.student_group.clone(),
            teacher: self.teacher.clone(),
            lesson_hour: hour,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Class {
    pub subject: Subject,
    pub student_group: StudentGroup,
    pub teacher: Teacher,
    pub lesson_hour: RepeatingLessonHour,
}

#[derive(Debug, PartialEq, Eq, Clone, From, Into)]
pub struct Schedule(pub Vec<Class>);