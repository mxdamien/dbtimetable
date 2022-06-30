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

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config: Config = confy::load("dbtimetable")?;
    let apiclient = DbApiClient::new(config.clone());
    let xmlparser = XmlParser::new();
    let presenter = Box::new(TimetablePresenterConsole::new());
    let app = App::new(apiclient, xmlparser, config.clone(), presenter);
    app.run().await;

    Ok(())
}
