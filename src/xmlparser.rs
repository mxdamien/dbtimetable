use crate::timetable::Timetable;
pub trait XmlParser {
    fn get_timetable(&self, xml: &String) -> Result<Timetable, String>;
}
