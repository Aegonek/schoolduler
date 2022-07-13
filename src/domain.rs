#[cfg(test)]
mod tests;
pub mod log;

use serde::{Serialize, Deserialize};
use time::util::weeks_in_year;
use time::{Duration, Time, Weekday::{self, *}, OffsetDateTime};
use num::Integer;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct Teacher {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct StudentGroup {
    pub year: u16,
    pub suffix: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub required_yearly_hours: u32,
}

impl Subject {
    pub fn required_weekly_hours(&self) -> u32 {
        let current_year = OffsetDateTime::now_utc().year();
        Integer::div_ceil(&self.required_yearly_hours, &(weeks_in_year(current_year) as u32))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct LessonHour {
    pub weekday: Weekday,
    pub time: Time,
    pub duration: Duration,
}

impl PartialOrd for LessonHour {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        const WEEKDAYS: [Weekday; 7] = [Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday];
        let cmp_weekdays = Ord::cmp(
            &WEEKDAYS.iter().position(|&x| x == self.weekday).unwrap(), 
            &WEEKDAYS.iter().position(|&x| x == other.weekday).unwrap());
        Some(cmp_weekdays.then(self.time.cmp(&other.time)))
    }
}

impl Ord for LessonHour {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Class {
    pub subject: Subject,
    pub student_group: StudentGroup,
    pub teacher: Teacher,
    pub lesson_hour: LessonHour,
}

impl Class {
    pub fn course(&self) -> Course {
        Course { subject: self.subject.clone(), student_group: self.student_group.clone(), teacher: self.teacher.clone() }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Course {
    pub subject: Subject,
    pub student_group: StudentGroup,
    pub teacher: Teacher,
}

impl Course {
    pub fn schedule_for(&self, hour: LessonHour) -> Class {
        Class {
            subject: self.subject.clone(),
            student_group: self.student_group.clone(),
            teacher: self.teacher.clone(),
            lesson_hour: hour,
        }
    }
}

pub fn standard_lesson_hours() -> Vec<LessonHour> {
    [Monday, Tuesday, Wednesday, Thursday, Friday]
        .into_iter()
        .flat_map(|weekday| {
            let mut hours: Vec<LessonHour> = Vec::new();
            let mut current_hour = Time::from_hms(8, 0, 0).unwrap();
    
            let day_end_hour = Time::from_hms(17, 0, 0).unwrap();
            const LESSON_DURATION: Duration = Duration::minutes(45);
            const BREAK_DURATION: Duration = Duration::minutes(10);
    
            while current_hour < day_end_hour {
                let next_hour = LessonHour {
                    weekday: weekday,
                    time: current_hour,
                    duration: LESSON_DURATION,
                };
                hours.push(next_hour);
                current_hour += LESSON_DURATION;
                current_hour += BREAK_DURATION;
            }
    
            hours
        })
        .collect()
}

pub type Requirements = Vec<Course>;

pub type Schedule = Vec<Class>;