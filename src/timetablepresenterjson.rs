use crate::timetable::{ArrivalDeparture, Timetable};
use crate::timetablepresenter::TimetablePresenter;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Serialize, Deserialize)]
struct TimetableStop {
    station: String,
    train: String,
    end_station: String,
    planned_time: String,
    actual_time: String,
}

pub struct TimetablePresenterJson();

impl TimetablePresenterJson {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl TimetablePresenter for TimetablePresenterJson {
    fn present(&self, timetable: &Timetable, eva: &String) {
        let station = get_station_name(timetable);
        let stops = get_stops(timetable, &station);
        let json = to_json(&stops);
        write_to_file(&eva, &json);
    }
}

fn get_station_name(timetable: &Timetable) -> String {
    return format!(
        "{}",
        timetable
            .station
            .as_ref()
            .unwrap_or(&"Station name missing".to_string())
    );
}

fn get_stops(timetable: &Timetable, station: &String) -> Vec<TimetableStop> {
    let mut stops = Vec::new();
    timetable
        .s
        .as_ref()
        .unwrap()
        .iter()
        .for_each(|s| match &s.dp {
            Some(dp) => match &s.tl {
                Some(tl) => {
                    let train_name = get_train_name(tl, dp);
                    let end_station = get_train_end_station(dp);
                    let planned_time = get_planned_time(dp);
                    let changed_time = get_changed_time(dp);

                    stops.push(TimetableStop {
                        station: station.clone(),
                        train: train_name.clone(),
                        end_station: end_station.clone(),
                        planned_time: planned_time.clone(),
                        actual_time: changed_time.clone(),
                    });
                }
                None => {}
            },
            None => {}
        });

    stops
}

fn get_train_name(
    tl: &crate::timetable::Triplabel,
    dp: &crate::timetable::ArrivalDeparture,
) -> String {
    let c =
        tl.c.as_ref()
            .unwrap_or(tl.n.as_ref().unwrap_or(&"".to_string()))
            .to_string();
    let l =
        dp.l.as_ref()
            .unwrap_or(tl.n.as_ref().unwrap_or(&"".to_string()))
            .to_string();

    return match (!c.is_empty(), !l.is_empty()) {
        (true, true) => format!("{}{}", c, l),
        _ => "Train name missing".to_string(),
    };
}

fn get_train_end_station(dp: &ArrivalDeparture) -> String {
    return format!(
        "{}",
        dp.ppth
            .as_ref()
            .unwrap_or(&"".to_string())
            .split('|')
            .last()
            .unwrap_or(&"".to_string())
    );
}

fn get_planned_time(dp: &ArrivalDeparture) -> String {
    return format!(
        "{}",
        NaiveDateTime::parse_from_str(dp.pt.as_ref().unwrap_or(&"-".to_string()), "%y%m%d%H%M")
            .unwrap()
            .to_string()
    );
}

fn get_changed_time(dp: &crate::timetable::ArrivalDeparture) -> String {
    match NaiveDateTime::parse_from_str(dp.ct.as_ref().unwrap_or(&"-".to_string()), "%y%m%d%H%M") {
        Ok(dt) => return format!("{}", dt.to_string()),
        _ => return format!("No delay"),
    }
}

fn to_json(stops: &Vec<TimetableStop>) -> String {
    let j = serde_json::to_string(&stops);
    format!("{}", j.unwrap_or_default())
}

fn write_to_file(eva: &String, json: &String) {
    let filename = format!("{}.json", &eva);
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(json.as_bytes()).expect("Unable to write data");
}
