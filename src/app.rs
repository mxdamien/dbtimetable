use crate::{
    config::Config, dbapiclient::DbApiClient, timetablepresenter::TimetablePresenter,
    xmlparser::XmlParser,
};

pub struct App {
    apiclient: Box<dyn DbApiClient>,
    xmlparser: Box<dyn XmlParser>,
    config: Config,
    presenter: Box<dyn TimetablePresenter>,
}

impl App {
    fn construct_changes_endpoint(eva: &String) -> String {
        format!("fchg/{}", eva)
    }

    pub fn new(
        apiclient: Box<dyn DbApiClient>,
        xmlparser: Box<dyn XmlParser>,
        config: Config,
        presenter: Box<dyn TimetablePresenter>,
    ) -> Self {
        Self {
            apiclient,
            xmlparser,
            config,
            presenter,
        }
    }

    pub async fn run(self) {
        for eva in &self.config.evas {
            let timetable_changes = match self
                .apiclient
                .get(App::construct_changes_endpoint(eva))
                .await
            {
                Ok(s) => s,
                Err(_) => "".into(),
            };

            if let Ok(changes) = self.xmlparser.get_timetable(&timetable_changes) {
                self.presenter.present(&changes, eva);
            }
        }
    }
}
