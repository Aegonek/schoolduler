mod joe;

use rand::prelude::*;
use rand::Rng;

use crate::school::*;

const POPULATION_SIZE: u32 = 100; // number of schedules to operate on
const NO_ITERATIONS: u32 = 5_000_000;

pub fn solve(requirements: Requirements) -> Schedule {
    joe::solve(requirements)
}

fn random_schedule<R: Rng>(
    Requirements {
        lessons,
        open_hours,
    }: &Requirements,
    rng: &mut R,
) -> Schedule {
    let schedule: Schedule = lessons
        .iter()
        .cloned()
        .flat_map(|req| {
            let weekly = req.required_weekly_hours();

            (1..=weekly)
                .map(|_| {
                    let hour = open_hours.choose(rng).unwrap().clone();
                    req.schedule_for(hour)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into();

    schedule
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;

    #[test]
    #[ignore = "manual check"]
    fn check_initial_schedules() {
        let mut rng = StdRng::seed_from_u64(10);
        let schedules = (0..POPULATION_SIZE)
            .map(|_| random_schedule(&input::mock_requirements(), &mut rng))
            .collect::<Vec<_>>();

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
