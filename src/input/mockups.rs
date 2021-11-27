use itertools::iproduct;
use rand::prelude::*;
use rand::SeedableRng;
use std::iter;

use crate::school::*;

const YEARS: u16 = 8;

fn mock_student_groups() -> Vec<StudentGroup> {
    iproduct!(0..=YEARS, 'a'..='f')
        .map(|(year, sfx)| StudentGroup {
            year: year,
            sufix: sfx.to_string(),
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct AnnotatedSubject {
    subject: Subject,
    for_year: u16,
    required_yearly_hours: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct AnnotatedTeacher {
    teacher: Teacher,
    subjects: Vec<AnnotatedSubject>,
}

fn mock_subjects() -> Vec<AnnotatedSubject> {
    fn subjects_for_year(year: u16) -> Vec<AnnotatedSubject> {
        vec![
            AnnotatedSubject {
                subject: Subject {
                    name: "Język polski".into(),
                },
                for_year: year,
                required_yearly_hours: 60,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Historia".into(),
                },
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject { name: "WoS".into() },
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Język angielski".into(),
                },
                for_year: year,
                required_yearly_hours: 60,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Matematyka".into(),
                },
                for_year: year,
                required_yearly_hours: 60,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Fizyka".into(),
                },
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Chemia".into(),
                },
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject {
                    name: "Biologia".into(),
                },
                for_year: year,
                required_yearly_hours: 30,
            },
            AnnotatedSubject {
                subject: Subject { name: "WF".into() },
                for_year: year,
                required_yearly_hours: 30,
            },
        ]
    }

    (0..=YEARS).flat_map(|i| subjects_for_year(i)).collect()
}

fn shuffling<T: 'static>(
    mut iterable: Vec<T>,
    mut rng: impl Rng + 'static,
) -> impl Iterator<Item = T> {
    iterable.shuffle(&mut rng);
    let mut iter = iterable.into_iter();

    let shuffling = iter::from_fn(move || {
        let next: Option<T> = iter.next();
        if next.is_some() {
            next
        } else {
            iterable.shuffle(&mut rng);
            iter = iterable.into_iter();
            iter.next()
        }
    });

    shuffling
}

fn mock_teachers() -> Vec<Teacher> {
    let mut rand = StdRng::seed_from_u64(32);

    let subjects = mock_subjects();
    let teacher_first_names = [
        "Piotr", "Adam", "Maciej", "Karolina", "Kornelia", "Kamila", "Magda", "Tomasz", "Filemon",
        "Rafał",
    ];

    let teacher_last_names = [
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

    todo!()
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
}
