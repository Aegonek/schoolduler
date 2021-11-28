use rand::distributions::{Uniform};
use rand::prelude::*;
use rand::Rng;

use crate::input;
use crate::school::*;

const POPULATION_SIZE: u32 = 100; // number of schedules to operate on

// trait Encode<T> {
//     fn encode(self) -> T;
// }

// impl Encode<Vec<u32>> for Schedule {
//     fn encode(self) -> Vec<u32> {
//         todo!()
//     }
// }

pub fn initialize_with_random_schedules(
    requirements: Vec<LessonBlock>,
    mut rng: impl Rng,
) -> Vec<Schedule> {
    let possible_lesson_hours = input::standard_open_hours();
    let lesson_hours_count = possible_lesson_hours.len();
    let hours_distribution = Uniform::from(0..lesson_hours_count);
    let mut schedules: Vec<Schedule> = Vec::with_capacity(
        POPULATION_SIZE
            .try_into()
            .expect("Weird issue with usize casting."),
    );

    for _ in 0..POPULATION_SIZE {
        let schedule: Schedule = requirements
            .iter()
            .cloned()
            .flat_map(|req| {
                let weekly = req.required_weekly_hours();

                (1..=weekly)
                    .map(|_| {
                        let i = hours_distribution.sample(&mut rng);
                        let hour = possible_lesson_hours[i];
                        Class {
                            subject: req.subject.clone(),
                            student_group: req.student_group.clone(),
                            teacher: req.teacher.clone(),
                            lesson_hour: hour,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into();

        schedules.push(schedule);
    }

    schedules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "manual check"]
    fn check_initial_schedules() {
        let schedules =
            initialize_with_random_schedules(input::mock_requirements(), StdRng::seed_from_u64(10));
        let lesson_count = schedules
            .iter()
            .map(|Schedule(lessons)| lessons.len())
            .sum::<usize>();
        println!(
            "There are {:?} schedules; which together have {:?} classes. So each of them should have {:?} lessons.",
            schedules.len(),
            lesson_count,
            lesson_count / schedules.len()
        )
    }
}
