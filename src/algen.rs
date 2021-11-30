use rand::distributions::Uniform;
use rand::prelude::*;
use rand::Rng;

use crate::input;
use crate::school::*;

const POPULATION_SIZE: u32 = 100; // number of schedules to operate on

#[derive(Debug, PartialEq, Eq, Clone)]
struct LessonInfo {
    pub subject: Subject,
    pub student_group: StudentGroup,
    pub teacher: Teacher,
}

// encoding a value yields a tuple of:
// - encoded value (genotype), specialized for easy genetic algorithms operation - mutation, crossover...
// - decoder, which can turn encoded value back into Schedule by storing related data
trait Encode<T>
where
    Self: Sized,
{
    type AssociatedDecoder: Decoder<T, Self>;

    fn encode(self) -> (T, Self::AssociatedDecoder);
}

impl Encode<Vec<u32>> for Schedule {
    type AssociatedDecoder = U32Decoder;
    // decoder is the immutable register of lessons info `info_index = Vec<(Subject, StudentGroup, Teacher)>`, immutable register of lesson hours `hour_index = Vec<RepeatingLessonHour>`
    // we encode info about hours into hours = Vec<i32>, gdzie dla każdego hour == hours[i]:
    // i - info_index[i] is info about lesson
    // hour - hour_index[hour] is info about lesson time

    fn encode(self) -> (Vec<u32>, Self::AssociatedDecoder) {
        let lessons = self.0;
        let all_available_hours = input::standard_open_hours(); // hardcoding available hours for now, TODO: let user pass it from input
        let (lesson_infos_in_sch, lesson_hours_in_sch): (Vec<_>, Vec<_>) = lessons
            .iter()
            .map(
                |Class {
                     subject,
                     student_group,
                     teacher,
                     lesson_hour,
                 }| {
                    (
                        LessonInfo {
                            subject: subject.clone(),
                            student_group: student_group.clone(),
                            teacher: teacher.clone(),
                        },
                        lesson_hour.clone(),
                    )
                },
            )
            .unzip();

        let hour_indices = lesson_hours_in_sch
            .iter()
            .map(|hour_schedule| {
                all_available_hours
                    .iter()
                    .position(|hour_in_list| hour_in_list == hour_schedule)
                    .map(|hour| hour as u32)
            })
            .collect::<Option<Vec<_>>>()
            .expect("ERROR: application generated impossible hour!");

        let decoder = U32Decoder {
            info_index: lesson_infos_in_sch,
            hour_index: all_available_hours,
        };

        (hour_indices, decoder)
    }
}

struct U32Decoder {
    pub info_index: Vec<LessonInfo>,
    pub hour_index: Vec<RepeatingLessonHour>,
}

trait Decoder<I, O> {
    fn decode(&self, encoded: I) -> O;
}

impl Decoder<Vec<u32>, Schedule> for U32Decoder {
    fn decode(&self, encoded: Vec<u32>) -> Schedule {
        let &U32Decoder {
            info_index,
            hour_index,
        } = &self;

        encoded
            .into_iter()
            .enumerate()
            .map(|(i, val)| {
                let lesson_info = info_index[i].clone();
                Class {
                    subject: lesson_info.subject.clone(),
                    student_group: lesson_info.student_group.clone(),
                    teacher: lesson_info.teacher.clone(),
                    lesson_hour: hour_index[val as usize],
                }
            })
            .collect::<Vec<_>>()
            .into()
    }
}

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

// TODO: test decoding and encoding
// - out of bounds errors, all numbers line up
// - encoding and decoding preserves data

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
