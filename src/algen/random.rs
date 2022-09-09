use crate::domain::*;
use rand::prelude::*;

pub fn random_schedule(requirements: &Requirements) -> Schedule {
    let mut rng = thread_rng();

    requirements
        .iter()
        .cloned()
        .flat_map(|req| {
            let weekly = req.subject.required_weekly_hours();

            (1..=weekly)
                .map(|_| {
                    let hour = standard_lesson_hours().choose(&mut rng).unwrap().clone();
                    req.schedule_for(hour)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
