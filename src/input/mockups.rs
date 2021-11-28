use crate::school::*;
use itertools::{iproduct, izip};

const YEARS: u16 = 8;
const TEACHER_HOURS_IN_WEEK: u16 = 40;
const WEEKS_IN_YEAR: u32 = 40;

const fn required_hours_in_week(required_hours_in_year: u32) -> u32 {
    required_hours_in_year / WEEKS_IN_YEAR
}

fn mock_student_groups() -> Vec<StudentGroup> {
    iproduct!(0..=YEARS, 'a'..='f')
        .map(|(year, sfx)| StudentGroup {
            year: year,
            sufix: sfx.to_string(),
        })
        .collect()
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct AnnotatedTeacher {
    teacher: Teacher,
    niche: Niche,
    years: Vec<u16>,
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

    (0..=YEARS).flat_map(|i| subjects_for_year(i)).collect()
}

fn mock_teachers() -> Vec<AnnotatedTeacher> {
    let first_names = [
        "Piotr", "Adam", "Maciej", "Karolina", "Kornelia", "Kamila", "Magda", "Tomasz", "Filemon",
        "Rafał",
    ];

    let last_names = [
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

    let names = izip!(first_names.into_iter(), last_names.into_iter())
        .chain(izip!(first_names.into_iter(), last_names.into_iter().rev()));

    use self::Niche::*;
    names
        .enumerate()
        .map(|(i, (first, last))| AnnotatedTeacher {
            teacher: Teacher {
                first_name: first.into(),
                last_name: last.into(),
            },
            niche: match i {
                0..=8 => Humanities,
                9..=16 => Sciences,
                17..=20 => PE,
                _ => panic!("This is really bad handling of it. But I'm done for now."),
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

pub fn mock_lesson_blocks() -> Vec<LessonBlock> {
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
            hours_assigned += required_hours_in_week(subject.required_yearly_hours);

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

    lesson_blocks
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
    #[ignore = "manual check"]
    fn check_lesson_blocks() {
        let lesson_blocks = mock_lesson_blocks();
        println!("Lesson blocks: {:?}. There are {:?} lesson blocks.", lesson_blocks, lesson_blocks.len())
    }
}
