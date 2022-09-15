use std::cmp;
use std::error::Error;

use itertools::Itertools;
use xlsxwriter::Workbook;
use xlsxwriter::Worksheet;
use xlsxwriter::WorksheetCol;
use xlsxwriter::WorksheetRow;
use xlsxwriter::XlsxError;

use crate::domain::Schedule;
use crate::log::{log, Logger};
use crate::utils::time;

pub fn save_schedule(schedule: &Schedule, logger: &mut Logger) -> Result<(), Box<dyn Error>> {
    log!(logger, "Saving schedule to .xlsx files...")?;
    write_by_student_group_sheet(schedule)?;
    write_by_teachers_sheet(schedule)?;
    log!(logger, "Finished saving schedules.")?;
    Ok(())
}

pub fn write_by_student_group_sheet(schedule: &Schedule) -> Result<(), Box<dyn Error>> {
    let filename = time::timestamped("output/schedules_by_student_groups.xlsx")
        .to_str()
        .expect("Expecting paths to contain only valid Unicode characters.")
        .to_owned();
    let workbook = Workbook::new(&filename);
    let mut schedule = schedule.clone();
    schedule.sort_by(|x, y| {
        Ord::cmp(&x.student_group.year, &y.student_group.year)
            .then(Ord::cmp(&x.student_group.suffix, &y.student_group.suffix))
    });

    let by_student_group =
        Itertools::group_by(schedule.into_iter(), |cls| cls.student_group.clone());
    for (group, classes) in by_student_group.into_iter() {
        let mut worksheet = workbook.add_worksheet(Some(&group.to_string()))?;
        let mut cursor = RowCursor::new(&mut worksheet);
        let mut classes: Box<[_]> = FromIterator::from_iter(classes);
        classes.sort_by_key(|x| x.lesson_hour);

        let by_days = Itertools::group_by(classes.into_iter(), |cls| cls.lesson_hour.weekday);
        for (day, classes) in by_days.into_iter() {
            cursor.write_string(&day.to_string())?;
            cursor.advance_row();
            for class in classes {
                cursor.write_string(&class.subject.to_string())?;
                cursor.write_string(&class.teacher.to_string())?;
                cursor.write_string(&class.lesson_hour.format_hour())?;
                cursor.advance_row();
            }
            cursor.advance_col();
        }
    }

    Ok(())
}

pub fn write_by_teachers_sheet(schedule: &Schedule) -> Result<(), Box<dyn Error>> {
    let filename = time::timestamped("output/schedules_by_teachers.xlsx")
        .to_str()
        .expect("Expecting paths to contain only valid Unicode characters.")
        .to_owned();
    let workbook = Workbook::new(&filename);
    let mut schedule = schedule.clone();
    schedule.sort_by_key(|cls| cls.teacher.clone());

    let by_teachers = Itertools::group_by(schedule.into_iter(), |cls| cls.teacher.clone());
    for (teacher, classes) in by_teachers.into_iter() {
        let mut worksheet = workbook.add_worksheet(Some(&teacher.to_string()))?;
        let mut cursor = RowCursor::new(&mut worksheet);
        let mut classes: Box<[_]> = FromIterator::from_iter(classes);
        classes.sort_by_key(|x| x.lesson_hour);

        let by_days = Itertools::group_by(classes.into_iter(), |cls| cls.lesson_hour.weekday);
        for (day, classes) in by_days.into_iter() {
            cursor.write_string(&day.to_string())?;
            cursor.advance_row();
            for class in classes {
                cursor.write_string(&class.subject.to_string())?;
                cursor.write_string(&class.student_group.to_string())?;
                cursor.write_string(&class.lesson_hour.format_hour())?;
                cursor.advance_row();
            }
            cursor.advance_col();
        }
    }

    Ok(())
}

// Wrapper for more convenient writing to .xlsx files. Only implementing what we need.
// Cursor is always on empty cell, methods advance column `after` writing.
pub struct RowCursor<'a> {
    worksheet: &'a mut Worksheet<'a>,
    cur_row: WorksheetRow,
    cur_col: WorksheetCol,
    max_col: WorksheetCol,
    pivot_col: WorksheetCol,
}

impl<'a> RowCursor<'a> {
    pub fn new(worksheet: &'a mut Worksheet<'a>) -> Self {
        RowCursor {
            worksheet,
            cur_row: 0,
            cur_col: 0,
            max_col: 0,
            pivot_col: 0,
        }
    }

    pub fn write_string(&mut self, text: &str) -> Result<(), XlsxError> {
        self.worksheet
            .write_string(self.cur_row, self.cur_col, text, None)?;
        self.max_col = cmp::max(self.cur_col, self.max_col);
        self.cur_col += 1;
        Ok(())
    }

    pub fn write_number(&mut self, number: f64) -> Result<(), XlsxError> {
        self.worksheet
            .write_number(self.cur_row, self.cur_col, number, None)?;
        self.cur_col += 1;
        self.max_col = cmp::max(self.cur_col, self.max_col);
        Ok(())
    }

    pub fn write_datetime(&mut self, datetime: &xlsxwriter::DateTime) -> Result<(), XlsxError> {
        self.worksheet
            .write_datetime(self.cur_row, self.cur_col, datetime, None)?;
        self.cur_col += 1;
        self.max_col = cmp::max(self.cur_col, self.max_col);
        Ok(())
    }

    pub fn advance_row(&mut self) {
        self.cur_row += 1;
        self.cur_col = self.pivot_col;
    }

    pub fn advance_col(&mut self) {
        self.pivot_col = self.max_col + 2;
        self.cur_col = self.pivot_col;
        self.cur_row = 0;
    }
}
