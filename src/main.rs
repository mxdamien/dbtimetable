mod app;
mod config;
mod dbapiclient;
mod dbapiclientimpl;
mod timetable;
mod timetableformatter;
mod timetableformatterjson;
mod timetableformattersimplestring;
mod xmlparser;
mod xmlparserquickxml;

use app::App;
use config::Config;
use dbapiclientimpl::DbApiClientImpl;
use timetableformatterjson::TimetableFormatterJson;
use timetableformattersimplestring::TimetableFormatterSimpleString;
use xmlparserquickxml::XmlParserQuickXml;

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

fn create_formatter(
    formattertype: &config::FormatterType,
) -> Box<dyn timetableformatter::TimetableFormatter> {
    match formattertype {
        config::FormatterType::SimpleString => Box::new(TimetableFormatterSimpleString::new()),
        config::FormatterType::Json => Box::new(TimetableFormatterJson::new()),
    }
}

fn create_xmlparser() -> Box<dyn xmlparser::XmlParser> {
    Box::new(XmlParserQuickXml::new())
}

fn create_apiclient(config: Config) -> Box<dyn dbapiclient::DbApiClient> {
    Box::new(DbApiClientImpl::new(config))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = load_config();
    let apiclient = create_apiclient(config.clone());
    let xmlparser = create_xmlparser();
    let formatter = create_formatter(&config.formatter);
    let app = App::new(apiclient, xmlparser, config.clone(), formatter);
    let str_table = app.run().await;
    print!("{}", str_table.unwrap_or("dbtimetable failed".to_string()));
    Ok(())
}
