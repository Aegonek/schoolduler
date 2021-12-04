mod joe;

use rand::prelude::*;
use rand::Rng;

use crate::school::*;

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