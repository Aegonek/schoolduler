mod mockups;

use time::{Duration, Time, Weekday};

use crate::school::*;

pub fn read_requirements_from_file() -> Vec<LessonBlock> {
    todo!()
}

pub fn mock_requirements() -> Requirements {
    mockups::mock_requirements()
}

pub fn standard_open_hours() -> Vec<RepeatingLessonHour> {
    fn hours_in_day(weekday: Weekday) -> Vec<RepeatingLessonHour> {
        let mut hours: Vec<RepeatingLessonHour> = Vec::new();
        let mut current_hour = Time::from_hms(8, 0, 0).unwrap();

        let day_end_hour = Time::from_hms(17, 0, 0).unwrap();
        const LESSON_DURATION: Duration = Duration::minutes(45);
        const BREAK_DURATION: Duration = Duration::minutes(10);

        while current_hour < day_end_hour {
            let next_hour = RepeatingLessonHour {
                weekday: weekday,
                time: current_hour,
                duration: LESSON_DURATION,
            };
            hours.push(next_hour);
            current_hour += LESSON_DURATION;
            current_hour += BREAK_DURATION;
        }

        hours
    }

    use Weekday::*;

    [Monday, Tuesday, Wednesday, Thursday, Friday]
        .into_iter()
        .flat_map(hours_in_day)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore="manual check"]
    fn check_standard_open_hours() {
        let hours = standard_open_hours();
        println!("{:?}", hours)
    }
}