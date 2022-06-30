use crate::timetable::Timetable;
use quick_xml::DeError;
pub struct XmlParser {}

impl XmlParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_timetable(&self, xml: &String) -> Result<Timetable, DeError> {
        match quick_xml::de::from_str::<Timetable>(xml.as_str())
        {
            Ok(tt) => Ok(tt),
            Err(err) => {
                println!("Timetable parsing error: {}", err); Err(err)
            }
        }
    }
}
