use super::Class;
use crate::algen::parametrized::execution::RunId;
use crate::db::DB_CONN;
use crate::utils::exts::result::ResultExt;
use crate::utils::log::DbWrite;
use rusqlite::params;
use serde_json;
use std::fmt::Display;

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Subject: {:?}\nStudent group: {:?}\nTeacher: {:?}\nLesson hour: {:?}",
            self.subject, self.student_group, self.teacher, self.lesson_hour
        ))?;
        Ok(())
    }
}

impl DbWrite for Class {
    type Context = RunId;

    fn write_db(&self, ctx: Self::Context) -> rusqlite::Result<()> {
        const SQL: &'static str = "
            INSERT INTO THETA_RESULTS (run, lesson)
            VALUES (?1, ?2)
        ";
        DB_CONN
            .lock().unwrap()
            .execute(SQL, params![ctx.0, serde_json::to_string(self).unwrap()])
            .void()
    }
}
