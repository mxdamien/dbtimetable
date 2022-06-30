## dbtimetable
A small Rust application to query the Deutsche Bahn (DB) timetable API. It can be used to display changes for a set of train stations

## Setup
You need to register with the DB API Marketplace to use the timeable API. 
[Link to API description](https://developers.deutschebahn.com/db-api-marketplace/apis/product/timetables/api/26494#/Timetables_10213/overview) 
You also need to obtain a Client ID and an API Key for the timeable API and set them in your local configuration. [config.rs](src/config.rs)

## Configure your stations
Stations are identified by their EVA numbers. The stations you want to observe have to be configured in your local configuration [config.rs](src/config.rs)
A list of stations alongside their EVA numbers can be found here: [Link](https://data.deutschebahn.com/dataset/data-haltestellen.html)

## License

[License](LICENSE.md)