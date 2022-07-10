use std::fmt::Display;

use rusqlite::params;

use crate::DB_CONN;
use crate::utils::log::DbWrite;
use crate::utils::result_ext::ResultExt;
use super::Class;
use serde_json;

impl Class {
    pub fn with_run(&self, run: usize) -> (usize, &Self) {
        (run, self)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Subject: {:?}\nStudent group: {:?}\nTeacher: {:?}\nLesson hour: {:?}", self.subject, self.student_group, self.teacher, self.lesson_hour))?;
        Ok(())
    }
}

impl DbWrite for (usize, &Class) {
    fn write_db(&self) -> rusqlite::Result<()> {
        DB_CONN.lock().unwrap().execute("
            INSERT INTO THETA_RESULTS (run, lesson)
            VALUES (?1, ?2)
        ", params![self.0, serde_json::to_string(self.1).unwrap()]
        ).void()
    }
}