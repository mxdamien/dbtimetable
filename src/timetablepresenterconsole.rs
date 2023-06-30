use crate::timetable::{ArrivalDeparture, Timetable};
use crate::timetablepresenter::TimetablePresenter;
use chrono::NaiveDateTime;

pub struct TimetablePresenterConsole();

impl TimetablePresenterConsole {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl TimetablePresenter for TimetablePresenterConsole {
    fn present(&self, timetable: &Timetable, _eva: &str) {
        print_station_name(timetable);
        print_seperator_lines(2);
        print_timetablestop(timetable);
        print_seperator_lines(1);
    }
}

fn print_timetablestop(timetable: &Timetable) {
    if let Some(s) = &timetable.s {
        for item in s.iter() {
            print_departure(item);
        }
    } else {
        println!("Timetable is empty.");
    }
}

fn print_departure(s: &crate::timetable::TimetableStop) {
    if let Some(dp) = &s.dp {
        if let Some(tl) = &s.tl {
            print_train_info(tl, dp);
            print_time_info(dp);
            print_seperator_lines(1);
        }
    }
}

fn print_time_info(dp: &ArrivalDeparture) {
    print_planned_time(dp);
    print_changed_time(dp);
}

fn print_planned_time(dp: &ArrivalDeparture) {
    let pt = dp.pt.as_deref().unwrap_or("-");
    if let Ok(dt) = NaiveDateTime::parse_from_str(pt, "%y%m%d%H%M") {
        println!("Planned time: {}", dt);
    }
}

fn print_changed_time(dp: &crate::timetable::ArrivalDeparture) {
    let ct = dp.ct.as_deref().unwrap_or("-");
    if let Ok(dt) = NaiveDateTime::parse_from_str(ct, "%y%m%d%H%M") {
        println!("Actual time: {}", dt);
    } else {
        println!("Actual time: No delay");
    }
}

fn print_train_info(tl: &crate::timetable::Triplabel, dp: &crate::timetable::ArrivalDeparture) {
    println!(
        "{} {}: {}",
        tl.c.as_ref().unwrap_or(&"".to_string()),
        dp.l.as_ref().unwrap_or(&"".to_string()),
        dp.ppth
            .as_ref()
            .unwrap_or(&"".to_string())
            .split('|')
            .last()
            .unwrap_or("")
    );
}

fn print_station_name(timetable: &Timetable) {
    println!(
        "{}",
        timetable
            .station
            .as_ref()
            .unwrap_or(&"Station name missing".to_string())
    );
}

fn print_seperator_lines(count: i16) {
    for _n in 0..count {
        println!("----------------------");
    }
}
