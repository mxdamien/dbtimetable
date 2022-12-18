use crate::timetable::Timetable;
use crate::xmlparser::XmlParser;

pub struct XmlParserQuickXml {}

impl XmlParserQuickXml {
    pub fn new() -> Self {
        Self {}
    }
}

impl XmlParser for XmlParserQuickXml {
    fn get_timetable(&self, xml: &String) -> Result<Timetable, String> {
        match quick_xml::de::from_str::<Timetable>(xml.as_str()) {
            Ok(tt) => Ok(tt),
            Err(err) => {
                println!("Timetable parsing error: {}", err);
                Err(format!("Timetable parsing error: {}", err))
            }
        }
    }
}
