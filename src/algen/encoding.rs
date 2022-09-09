use bitvec::vec::BitVec;

use super::Chromosome;
use crate::domain::*;

pub struct Decoder {
    courses: Vec<Course>,
    hours: Vec<LessonHour>,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            courses: Vec::new(),
            hours: Vec::new(),
        }
    }

    pub fn encode(&mut self, raw: &Schedule) -> Chromosome {
        let mut res: Vec<u8> = Vec::with_capacity(raw.len());

        self.hours = standard_lesson_hours();
        for class in raw.into_iter().cloned() {
            self.courses.push(class.course());
            let value = self
                .hours
                .iter()
                .position(|&hour| hour == class.lesson_hour)
                .unwrap()
                .to_be_bytes();
            if value[0..7].into_iter().any(|byte| byte.count_ones() != 0) {
                panic!("We had more than 255 available hours in the week! Crashing the program.")
            }
            res.push(value[7]);
        }

        Chromosome(BitVec::from_vec(res))
    }

    pub fn decode(&self, encoded: &Chromosome) -> Schedule {
        encoded
            .0
            .clone()
            .into_vec()
            .into_iter()
            .enumerate()
            .map(|(i, val)| {
                let course = self.courses[i].clone();
                let hour_i = val as usize % self.hours.len();
                let hour = self.hours[hour_i];
                course.schedule_for(hour)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algen::random;
    use serde_json;

    #[test]
    fn decoded_eq_encoded() {
        let raw = include_str!("../../input/example-courses.json");
        let required: Vec<Course> = serde_json::from_str(raw).unwrap();
        let schedule = random::random_schedule(&required);
        let mut decoder = Decoder::new();
        let encoded = decoder.encode(&schedule);
        let decoded = decoder.decode(&encoded);
        eprintln!("Comparing original schedule with schedule after encoding and decoding...");
        eprintln!("First 5 lessons before encoding: {:?}", &schedule[0..5]);
        eprintln!("First 5 lessons after decoding: {:?}", &decoded[0..5]);

        assert!(decoded == schedule);
    }
}
