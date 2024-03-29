use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum FormatterType {
    SimpleString,
    Json,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub url: String,
    pub client_id: String,
    pub api_key: String,
    pub evas: Vec<String>,
    pub formatter: FormatterType,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            url: "https://apis.deutschebahn.com/db-api-marketplace/apis/timetables/v1/".to_string(),
            client_id: "123456789".to_string(),
            api_key: "123456789".to_string(),
            evas: vec!["8003368".to_string()],
            formatter: FormatterType::Json,
        }
    }
}
