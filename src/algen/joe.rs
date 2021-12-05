use itertools::Itertools;
use once_cell::unsync::Lazy;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::mem;
use std::ops::Range;
use tap::{Conv, Tap, TryConv};

use super::*;
use crate::school::*;

/// Joe is take on algorithms with following assumptions:  
/// A gene is represented by unsigned integer, which can be decoded to specific time.  
/// NATURAL NUMBERS REPRESENTATION  
/// We don't enforce that genotype has no hard conflicts.  
/// Fitness function checks the number of hard conflicts in genotype.  
/// During decoding of genotype, we unfold hard conflicts using deterministic algorithm.  

const POPULATION_SIZE: u32 = 100; // number of schedules to operate on
const NO_ITERATIONS: u32 = 5_000_000;
const MUTATION_PROBABILITY: u32 = 5; // w promilach
const MUTATION_CREEP_DISTRIBUTION: Lazy<Uniform<i32>> = Lazy::new(|| Uniform::new_inclusive(-5, 5));
const PERFECT_SCORE: u32 = 1000;

pub fn solve(requirements: Requirements) -> Schedule {
    let initial: Vec<Schedule> = (0..POPULATION_SIZE)
        .map(|_| random_schedule(&requirements, &mut thread_rng()))
        .collect();
    let resource_map = make_resource_map(requirements);
    let hour_bounds = 0..(resource_map.hour_index.len() as u32);

    let mut genotypes: Vec<Chromosome> = initial
        .into_iter()
        .map(|sch| encode(sch, &resource_map))
        .collect(); // TODO: test that encoding works correctly.

    for _ in 0..NO_ITERATIONS {
        let rated = mem::replace(&mut genotypes, Vec::with_capacity(POPULATION_SIZE as usize));
        let rated = rated
            .into_iter()
            .map(|chr| {
                let rating = rate_fitness(&chr, &resource_map);
                (chr, rating)
            })
            .collect::<Vec<_>>();

        let parents = (0..POPULATION_SIZE)
            .map(|_| roulette_selection(&rated).clone())
            .collect::<Vec<_>>()
            .tap_mut(|xs| xs.shuffle(&mut thread_rng()));

        for (parent1, parent2) in parents.into_iter().tuples() {
            let (child1, child2) = one_point_crossover(parent1, parent2);
            for mut child in [child1, child2] {
                creep_mutation(&mut child, &hour_bounds);
                genotypes.push(child);
            }
        }
    }

    let best = genotypes
        .into_iter()
        .map(|chr| {
            let rating = rate_fitness(&chr, &resource_map);
            (chr, rating)
        })
        .max_by_key(|x| x.1)
        .unwrap()
        .0;

    decode(best, &resource_map)
}

fn creep_mutation(chromosome: &mut [u32], bounds: &Range<u32>) {
    for allel in chromosome {
        if thread_rng().gen_range(0..1000) < MUTATION_PROBABILITY {
            // TODO - analyze how all those thread_rngs would get along with concurrency
            let creep = MUTATION_CREEP_DISTRIBUTION.sample(&mut thread_rng());
            let mut raw = (*allel).conv::<i64>() + creep as i64;
            raw = max(bounds.start.into(), raw);
            raw = min(raw, bounds.end.into()); // TODO: test this
            *allel = raw.try_conv::<u32>().unwrap();
        }
    }
}

fn one_point_crossover(x: Chromosome, y: Chromosome) -> (Chromosome, Chromosome) {
    let axis = thread_rng().gen_range(0..x.len());
    let (x1, x2) = x.split_at(axis);
    let (y1, y2) = y.split_at(axis);
    let new_x = x1.to_vec().tap_mut(|x| x.extend(y2));
    let new_y = x2.to_vec().tap_mut(|x| x.extend(y1));
    (new_x, new_y)
}

fn roulette_selection(population: &Vec<(Chromosome, u32)>) -> &Chromosome {
    let sum_of_ratings: u32 = population.iter().map(|x| x.1).sum();
    let mut random: u32 = thread_rng().gen_range(0..sum_of_ratings);
    for (chromosome, rating) in population {
        if random < *rating {
            return chromosome;
        }
        random = random.saturating_sub(*rating);
    }

    panic!("Roulette selection should return one chromosome!")
}

/// Returns a number in range 0..1000
fn rate_fitness(genotype: &Chromosome, resource_map: &ResourceMap) -> u32 {
    let mut grouped_by_hour = genotype
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .into_group_map();

    grouped_by_hour.retain(|k, v| v.len() < 2); // drop elements where repetition cannot occur
    let mut conflicts: u32 = 0;
    for (&hour_i, lessons_i) in grouped_by_hour {
        conflicts += count_repetitions(hour_i, &lessons_i, resource_map, |class| class.teacher);
        conflicts += count_repetitions(hour_i, &lessons_i, resource_map, |class| class.student_group);
    }

    conflicts
}

/// Count times when teacher had to lead more than one lesson at the same time
fn count_repetitions<F, T>(
    hour_i: u32,
    lessons_i: &Vec<usize>,
    resource_map: &ResourceMap,
    select_key: F,
) -> u32
where
    F: Fn(Class) -> T,
    T: Hash + Eq,
{
    // doesn't that kinda throw my optimizations out of the window?
    let lessons: Vec<Class> = lessons_i
        .into_iter()
        .map(|&i| peek(i as usize, hour_i as usize, resource_map))
        .collect();

    let repetitions = lessons
        .into_iter()
        .fold(
            (0, HashSet::<T>::new()),
            |(repetitions, mut encountered), x| {
                let projected = (&select_key)(x);
                if (encountered.contains(&projected)) {
                    return (repetitions + 1, encountered);
                } else {
                    return (repetitions, {
                        encountered.insert(projected);
                        encountered
                    });
                }
            },
        )
        .0;

    repetitions
}

// position in vector points to LessonInfo (in ResourceMap)
// value points to LessonHour
type Chromosome = Vec<u32>;

fn encode(
    Schedule(mut classes): Schedule,
    ResourceMap {
        info_index,
        hour_index,
    }: &ResourceMap,
) -> Chromosome {
    // align lessons with resource_map.info_index in such a way, that lessons[i].conv::<LessonInfo>() == info_index[i]
    align(&mut classes, &info_index);
    let classes = classes
        .iter()
        .map(|Class { lesson_hour, .. }| {
            hour_index
                .iter()
                .position(|x| x == lesson_hour)
                .expect("Couldn't encode LessonHour! Something's wrong with resource map.")
                as u32
        })
        .collect::<Vec<_>>();

    classes
}

fn decode(
    genotype: Chromosome,
    ResourceMap {
        info_index,
        hour_index,
    }: &ResourceMap,
) -> Schedule {
    genotype
        .into_iter()
        .enumerate()
        .map(|(i, val)| {
            let lesson_info = info_index[i].clone();
            let lesson_hour = hour_index[val as usize];
            lesson_info.schedule_for(lesson_hour)
        })
        .collect::<Vec<_>>()
        .into()
}

fn peek(
    lesson_i: usize,
    hour_i: usize,
    ResourceMap {
        info_index,
        hour_index,
    }: &ResourceMap,
) -> Class {
    let lesson_info = &info_index[lesson_i];
    let lesson_hour = &hour_index[hour_i];
    let class = lesson_info.schedule_for(*lesson_hour);
    class
}

fn align(lessons: &mut Vec<Class>, info_index: &[LessonInfo]) {
    let mut aligned: Vec<Class> = Vec::with_capacity(lessons.len());
    for info in info_index {
        let corresponding = lessons
            .iter()
            .find(|&cls| {
                cls.subject == info.subject
                    && cls.student_group == info.student_group
                    && cls.teacher == info.teacher
            })
            .expect("We're expecting every class is registered in the index!");
        aligned.push(corresponding.clone());
    }

    let _ = mem::replace(lessons, aligned);
    // TODO: test!
}

fn make_resource_map(
    Requirements {
        lessons,
        open_hours,
    }: Requirements,
) -> ResourceMap {
    let lesson_infos: Vec<LessonInfo> = lessons.into_iter().map(|x| x.into()).collect();

    ResourceMap {
        info_index: lesson_infos,
        hour_index: open_hours,
    }
}

struct ResourceMap {
    pub info_index: Vec<LessonInfo>,
    pub hour_index: Vec<RepeatingLessonHour>,
}

#[cfg(test)]
mod tests {
    use crate::input::mockups;

    use super::*;

    fn initial_schedules() -> Vec<Schedule> {
        let mut rng = StdRng::seed_from_u64(10);
        (0..POPULATION_SIZE)
            .map(|_| random_schedule(&mockups::mock_requirements(), &mut rng))
            .collect::<Vec<_>>()
    }

    #[test]
    #[ignore = "manual check"]
    fn check_initial_schedules() {
        let schedules = initial_schedules();

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

    // AAA! What do I even check?
}
