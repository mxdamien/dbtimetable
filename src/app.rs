use crate::{
    config::Config, dbapiclient::DbApiClient, timetableformatter::TimetableFormatter,
    xmlparser::XmlParser,
};

pub struct App {
    apiclient: Box<dyn DbApiClient>,
    xmlparser: Box<dyn XmlParser>,
    config: Config,
    formatter: Box<dyn TimetableFormatter>,
}

impl App {
    fn construct_changes_endpoint(eva: &String) -> String {
        format!("fchg/{}", eva)
    }

    pub fn new(
        apiclient: Box<dyn DbApiClient>,
        xmlparser: Box<dyn XmlParser>,
        config: Config,
        formatter: Box<dyn TimetableFormatter>,
    ) -> Self {
        Self {
            apiclient,
            xmlparser,
            config,
            formatter,
        }
    }

    pub async fn run(self) -> Result<String, String> {
        let mut ret: String = "".to_string();
        for eva in &self.config.evas {
            let timetable = match self
                .apiclient
                .get(App::construct_changes_endpoint(eva))
                .await
            {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            };

            match self.xmlparser.get_timetable(&timetable) {
                Ok(timetable) => match self.formatter.format(&timetable) {
                    Ok(s) => ret.push_str(&s),
                    Err(e) => return Err(e.to_string()),
                },
                Err(e) => return Err(e.to_string()),
            }
        }
        Err("".to_string())
    }
}
