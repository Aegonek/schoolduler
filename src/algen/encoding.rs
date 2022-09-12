use super::{Chromosome, Gene};
use crate::domain::*;

pub struct Decoder {
    courses: Vec<Course>,
    hours: Vec<LessonHour>,
    teachers: Vec<Teacher>,
    student_groups: Vec<StudentGroup>,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            courses: Vec::new(),
            hours: Vec::new(),
            teachers: Vec::new(),
            student_groups: Vec::new(),
        }
    }

    pub fn encode(&mut self, schedule: &Schedule) -> Chromosome {
        // Initialize indices
        for class in schedule.into_iter() {
            let teacher_i = self.teachers.partition_point(|x| x < &class.teacher);
            match self.teachers.get(teacher_i) {
                Some(teacher) if teacher == &class.teacher => (),
                _ => self.teachers.insert(teacher_i, class.teacher.clone()),
            }

            let group_i = self
                .student_groups
                .partition_point(|x| x < &class.student_group);
            match self.student_groups.get(group_i) {
                Some(group) if group == &class.student_group => (),
                _ => self
                    .student_groups
                    .insert(group_i, class.student_group.clone()),
            }
        }

        let mut res: Vec<Gene> = Vec::with_capacity(schedule.len());
        self.hours = standard_lesson_hours();
        self.hours.sort();
        for class in schedule.into_iter() {
            self.courses.push(class.course());

            let hour_i = self
                .hours
                .binary_search(&class.lesson_hour)
                .expect("Unexpected error: Couldn't build lookup table!");
            assert!(hour_i <= u8::MAX.into(), "Unexpected error: We had more than 255 available hours in the week! Crashing the program.");

            let teacher_i = self
                .teachers
                .binary_search(&class.teacher)
                .expect("Unexpected error: Couldn't build lookup table!");
            assert!(teacher_i <= u8::MAX.into(), "Unexpected error: We had more than 255 teachers! Crashing the program.");

            let group_i = self
                .student_groups
                .binary_search(&class.student_group)
                .expect("Unexpected error: Couldn't build lookup table!");
            assert!(group_i <= u8::MAX.into(), "Unexpected error: We had more than 255 student groups! Crashing the program.");

            let gene = Gene {
                hour: hour_i as u8,
                teacher: teacher_i as u8,
                student_group: group_i as u8,
            };
            res.push(gene);
        }

        Chromosome(res)
    }

    pub fn decode(&self, encoded: &Chromosome) -> Schedule {
        encoded
            .0
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, gene)| {
                let course = self.courses[i].clone();
                let hour_i = gene.hour as usize % self.hours.len();
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
