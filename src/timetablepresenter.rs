use crate::timetable::Timetable;

pub trait TimetablePresenter {
    fn present(&self, timetable: &Timetable, eva: &String);
}
