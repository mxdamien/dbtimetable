mod app;
mod config;
mod dbapiclient;
mod timetable;
mod timetablepresenter;
mod timetablepresenterconsole;
mod xmlparser;

use app::App;
use config::Config;
use dbapiclient::DbApiClient;
use timetablepresenterconsole::TimetablePresenterConsole;
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

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = load_config();
    let apiclient = DbApiClient::new(config.clone());
    let xmlparser = XmlParser::new();
    let presenter = Box::new(TimetablePresenterConsole::new());
    let app = App::new(apiclient, xmlparser, config.clone(), presenter);
    app.run().await;

    Ok(())
}
