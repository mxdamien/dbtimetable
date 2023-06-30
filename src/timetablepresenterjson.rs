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
    fn present(&self, timetable: &Timetable, eva: &str) {
        let station = get_station_name(timetable);
        let stops = get_stops(timetable, &station);
        let json = to_json(&stops);
        write_to_file(eva, &json);
    }
}

fn get_station_name(timetable: &Timetable) -> String {
    if let Some(station) = &timetable.station {
        station.to_string()
    } else {
        "Station name missing".to_string()
    }
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
                        station: station.to_string(),
                        train: train_name,
                        end_station,
                        planned_time,
                        actual_time: changed_time,
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
    let c = tl.c.as_ref().unwrap_or(&"".to_string()).to_string();
    let l = dp.l.as_ref().unwrap_or(&"".to_string()).to_string();
    let n = tl.n.as_ref().unwrap_or(&"".to_string()).to_string();

    match (!c.is_empty(), !l.is_empty(), !n.is_empty()) {
        (false, false, true) => n,
        (false, true, true) => format!("{}{}", l, n),
        (true, false, true) => format!("{}{}", c, n),
        (true, true, _) => format!("{}{}", c, l),
        _ => "Train name missing".to_string(),
    }
}

fn get_train_end_station(dp: &ArrivalDeparture) -> String {
    dp.ppth
        .as_ref()
        .unwrap_or(&"".to_string())
        .split('|')
        .last()
        .unwrap_or("")
        .to_string()
}

fn get_planned_time(dp: &ArrivalDeparture) -> String {
    if let Some(pt) = &dp.pt {
        NaiveDateTime::parse_from_str(pt, "%y%m%d%H%M")
            .map(|dt| dt.to_string())
            .unwrap_or_else(|_| "-".to_string())
    } else {
        "-".to_string()
    }
}

fn get_changed_time(dp: &ArrivalDeparture) -> String {
    if let Some(ct) = &dp.ct {
        NaiveDateTime::parse_from_str(ct, "%y%m%d%H%M")
            .map(|dt| dt.to_string())
            .unwrap_or_else(|_| "No delay".to_string())
    } else {
        "No delay".to_string()
    }
}

fn to_json(stops: &Vec<TimetableStop>) -> String {
    let j = serde_json::to_string(&stops);
    j.unwrap_or_default()
}

fn write_to_file(eva: &str, json: &String) {
    let filename = format!("{}.json", &eva);
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(json.as_bytes()).expect("Unable to write data");
}
