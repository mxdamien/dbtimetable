use crate::timetable::Timetable;

pub trait TimetablePresenter {
    fn present(&mut self, timetable: &Timetable) -> Result<String, String>;
}
