use rand::prelude::*;
use tap::Tap;
use std::mem;
use itertools::Itertools;

use super::*;
use crate::school::*;

/// Joe is take on algorithms with following assumptions:  
/// A gene is represented by unsigned integer, which can be decoded to specific time.  
/// NATURAL NUMBERS REPRESENTATION  
/// We don't enforce that genotype has no hard conflicts.  
/// Fitness function checks the number of hard conflicts in genotype.  
/// During decoding of genotype, we unfold hard conflicts using deterministic algorithm.  

pub fn solve(requirements: Requirements) -> Schedule {
    let mut rng = thread_rng();
    let initial: Vec<Schedule> = (0..POPULATION_SIZE)
        .map(|_| random_schedule(&requirements, &mut rng))
        .collect();
    let resource_map = make_resource_map(requirements);
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
            
        for (parent1, parent2) in parents.into_iter().map(|x| x.0).tuples() {

        } 
    }

    todo!()
}

fn roulette_selection(population: &Vec<(Chromosome, u32)>) -> &(Chromosome, u32) {
    let sum_of_ratings: u32 = population.iter().map(|x| x.1).sum();
    todo!()
}

fn rate_fitness(chr: &[u32], resource_map: &ResourceMap) -> u32 {
    todo!()
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

// impl Decoder<JoeChromosome, Schedule> for JoeDecoder {
//     fn decode(&self, encoded: JoeChromosome) -> Schedule {
//         let JoeDecoder {
//             info_index,
//             hour_index,
//         } = self;

//         encoded
//             .conv::<Vec<u32>>()
//             .into_iter()
//             .enumerate()
//             .map(|(i, val)| {
//                 let lesson_info = info_index[i].clone();
//                 let lesson_hour = hour_index[val as usize];
//                 lesson_info.schedule_for(lesson_hour)
//             })
//             .collect::<Vec<_>>()
//             .into()
//     }
// }
