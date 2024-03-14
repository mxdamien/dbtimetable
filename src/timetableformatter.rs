use crate::timetable::Timetable;

pub trait TimetableFormatter {
    fn format(&self, timetable: &Timetable) -> Result<String, String>;
}
