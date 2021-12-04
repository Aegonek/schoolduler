use std::panic::AssertUnwindSafe;

use derive_more::{From, Into};

use rand::prelude::*;
use tap::Conv;

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
    let initial = (0..POPULATION_SIZE)
        .map(|_| random_schedule(&requirements, &mut rng))
        .collect::<Vec<_>>();
    todo!()
}

#[derive(Debug, PartialEq, Eq, From, Into)]
struct JoeChromosome(pub Vec<u32>);

struct JoeDecoder {
    pub info_index: Vec<LessonInfo>,
    pub hour_index: Vec<RepeatingLessonHour>,
}

impl Decoder<JoeChromosome, Schedule> for JoeDecoder {
    fn decode(&self, encoded: JoeChromosome) -> Schedule {
        let JoeDecoder {
            info_index,
            hour_index,
        } = self;

        encoded
            .conv::<Vec<u32>>()
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
}