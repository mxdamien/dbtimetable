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
    fn present(&self, timetable: &Timetable) {
        print_station_name(timetable);
        print_seperator_lines(2);
        print_timetablestop(timetable);
        print_seperator_lines(1);
    }
}

fn print_timetablestop(timetable: &Timetable) {
    timetable.s.as_ref().unwrap().iter().for_each(|s| {
        print_departure(s);
    });
}

fn print_departure(s: &crate::timetable::TimetableStop) {
    match &s.dp {
        Some(dp) => match &s.tl {
            Some(tl) => {
                print_train_info(tl, dp);
                print_time_info(dp);
                print_seperator_lines(1);
            }
            None => (),
        },
        None => (),
    }
}

fn print_time_info(dp: &ArrivalDeparture) {
    print_planned_time(dp);
    print_changed_time(dp);
}

fn print_planned_time(dp: &ArrivalDeparture) {
    println!(
        "Planned time: {}",
        NaiveDateTime::parse_from_str(dp.pt.as_ref().unwrap_or(&"-".to_string()), "%y%m%d%H%M")
            .unwrap()
            .to_string()
    );
}

fn print_changed_time(dp: &crate::timetable::ArrivalDeparture) {
    match NaiveDateTime::parse_from_str(dp.ct.as_ref().unwrap_or(&"-".to_string()), "%y%m%d%H%M") {
        Ok(dt) => println!("Actual time: {}", dt.to_string()),
        _ => println!("Actual time: No delay"),
    }
}

fn print_train_info(tl: &crate::timetable::Triplabel, dp: &crate::timetable::ArrivalDeparture) {
    println!(
        "{}{}: {}",
        tl.c.as_ref().unwrap_or(&"".to_string()),
        dp.l.as_ref()
            .unwrap_or(tl.n.as_ref().unwrap_or(&"".to_string())),
        dp.ppth
            .as_ref()
            .unwrap_or(&"".to_string())
            .split('|')
            .last()
            .unwrap_or(&"".to_string())
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
