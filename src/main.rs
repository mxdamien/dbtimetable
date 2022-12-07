mod app;
mod config;
mod dbapiclient;
mod timetable;
mod timetablepresenter;
mod timetablepresenterconsole;
mod timetablepresenterjson;
mod xmlparser;

use app::App;
use config::Config;
use dbapiclient::DbApiClient;
use timetablepresenterconsole::TimetablePresenterConsole;
use timetablepresenterjson::TimetablePresenterJson;
use xmlparser::XmlParser;

extern crate confy;

fn load_config() -> Config {
    match confy::load("dbtimetable", "config") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}. Using default config", e);
            Config::default()
        }
    }
}

fn create_presenter(
    presentertype: &config::PresenterType,
) -> Box<dyn timetablepresenter::TimetablePresenter> {
    match presentertype {
        config::PresenterType::Console => Box::new(TimetablePresenterConsole::new()),
        config::PresenterType::Json => Box::new(TimetablePresenterJson::new()),
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = load_config();
    let apiclient = DbApiClient::new(config.clone());
    let xmlparser = XmlParser::new();
    let presenter = create_presenter(&config.presenter);
    let app = App::new(apiclient, xmlparser, config.clone(), presenter);
    app.run().await;

    Ok(())
}
