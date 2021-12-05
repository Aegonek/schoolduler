use super::*;
use crate::school::*;

use itertools::{iproduct, izip};
use num::Integer;
use time::util::weeks_in_year;
use time::{OffsetDateTime, Weekday, Time, Duration};

const STUDENT_GROUP_YEARS: u16 = 8;
const TEACHER_HOURS_IN_WEEK: u16 = 40;

pub fn mock_requirements() -> Requirements {
    let students = mock_student_groups();
    let teachers = mock_teachers();
    let subjects = mock_subjects();

    let mut required_lessons = iproduct!(students, subjects)
        .filter(|(sgr, sub)| sgr.year == sub.for_year)
        .collect::<Vec<_>>();

    let mut lesson_blocks: Vec<LessonBlock> = Vec::with_capacity(required_lessons.len());

    for teacher in teachers {
        let mut hours_assigned = 0;
        for (group, subject) in required_lessons.drain_filter(|(group, subject)| {
            subject.niche == teacher.niche && teacher.years.contains(&group.year)
        }) {
            hours_assigned += subject.required_weekly_hours();

            let new_block = LessonBlock {
                subject: subject.subject,
                student_group: group,
                teacher: teacher.teacher.clone(),
                required_yearly_hours: subject.required_yearly_hours,
            };
            lesson_blocks.push(new_block);

            if hours_assigned >= TEACHER_HOURS_IN_WEEK.into() {
                break;
            }
        }
    }

    let open_hours = standard_open_hours();
    Requirements {
        lessons: lesson_blocks,
        open_hours: open_hours,
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Niche {
    Humanities,
    Sciences,
    PE,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct AnnotatedSubject {
    subject: Subject,
    niche: Niche,
    for_year: u16,
    required_yearly_hours: u32,
}

impl AnnotatedSubject {
    pub fn required_weekly_hours(&self) -> u32 {
        let current_year = OffsetDateTime::now_utc().year();
        self.required_yearly_hours
            .div_ceil(&u32::from(weeks_in_year(current_year)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct AnnotatedTeacher {
    teacher: Teacher,
    niche: Niche,
    years: Vec<u16>,
}

const fn const_div_ceil(x: usize, y: usize) -> usize {
    x / y + if x % y == 0 { 1 } else { 0 }
}

// TODO: read from some file later.

fn mock_student_groups() -> Vec<StudentGroup> {
    iproduct!(0..=STUDENT_GROUP_YEARS, 'a'..='f')
        .map(|(year, sfx)| StudentGroup {
            year: year,
            sufix: sfx.to_string(),
        })
        .collect()
}

fn mock_subjects() -> Vec<AnnotatedSubject> {
    use self::Niche::*;
    fn subjects_for_year(year: u16) -> Vec<AnnotatedSubject> {
        vec![
            AnnotatedSubject {
                subject: Subject {
                    name: "Język polski".into(),
                },
                niche: Humanities,
                for_year: year,
                required_yearly_hours: 60,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Historia".into(),
                },
                niche: Humanities,
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject { name: "WoS".into() },
                niche: Humanities,
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Język angielski".into(),
                },
                niche: Humanities,
                for_year: year,
                required_yearly_hours: 60,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Matematyka".into(),
                },
                niche: Sciences,
                for_year: year,
                required_yearly_hours: 60,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Fizyka".into(),
                },
                niche: Sciences,
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Chemia".into(),
                },
                niche: Sciences,
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Biologia".into(),
                },
                niche: Sciences,
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject { name: "WF".into() },
                niche: PE,
                for_year: year,
                required_yearly_hours: 30,
            },
        ]
    }

    (0..=STUDENT_GROUP_YEARS)
        .flat_map(|i| subjects_for_year(i))
        .collect()
}

fn mock_teachers() -> Vec<AnnotatedTeacher> {
    const FIRST_NAMES: [&str; 10] = [
        "Piotr", "Adam", "Maciej", "Karolina", "Kornelia", "Kamila", "Magda", "Tomasz", "Filemon",
        "Rafał",
    ];

    const LAST_NAMES: [&str; 10] = [
        "Kowalski",
        "Nowak",
        "Świr",
        "Kwiatkowski",
        "Lempart",
        "Kaczyński",
        "Kot",
        "Gałecki",
        "Szlachta",
        "Piorun",
    ];

    const NAMES_COUNT: usize = FIRST_NAMES.len() * 2;
    let names = izip!(FIRST_NAMES.into_iter(), LAST_NAMES.into_iter())
        .chain(izip!(FIRST_NAMES.into_iter(), LAST_NAMES.into_iter().rev()));

    use self::Niche::*;
    names
        .enumerate()
        .map(|(i, (first, last))| AnnotatedTeacher {
            teacher: Teacher {
                first_name: first.into(),
                last_name: last.into(),
            },
            niche: {
                const ONE_THIRD: usize = const_div_ceil(NAMES_COUNT, 3);
                const TWO_THIRDS: usize = ONE_THIRD * 2;
                match i {
                    0..ONE_THIRD => Humanities,
                    ONE_THIRD..TWO_THIRDS => Sciences,
                    TWO_THIRDS.. => PE,
                    _ => unreachable!("Should be unreachable, unless my math is bad."),
                }
            },
            years: match i % 3 {
                0 => vec![1, 2, 3],
                1 => vec![4, 5, 6],
                2 => vec![7, 8],
                _ => unreachable!(
                    "Unreachable code path - i % 3 cannot return number greater than 2!"
                ),
            },
        })
        .collect()
}

fn standard_open_hours() -> Vec<RepeatingLessonHour> {
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
    #[ignore = "manual check"]
    fn check_student_groups() {
        let groups = mock_student_groups();
        println!("{:?}", groups)
    }

    #[test]
    #[ignore = "manual check"]
    fn check_subjects() {
        let subjects = mock_subjects();
        println!("{:?}", subjects)
    }

    #[test]
    #[ignore = "manual check"]
    fn check_teachers() {
        let teachers = mock_teachers();
        println!("{:?}", teachers)
    }

    #[test]
    #[ignore="manual check"]
    fn check_standard_open_hours() {
        let hours = standard_open_hours();
        println!("{:?}", hours)
    }

    #[test]
    #[ignore = "manual check"]
    fn check_requirements() {
        let Requirements { lessons, open_hours } = mock_requirements();
        println!(
            "Lesson blocks: {:?}. There are {:?} lesson blocks.",
            lessons,
            lessons.len()
        );

        println!(
            "Open hours: {:?}. There are: {:?} open hours!",
            open_hours,
            open_hours.len()
        );
    }
}
