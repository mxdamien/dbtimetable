use crate::timetable::{ArrivalDeparture, Timetable};
use crate::timetablepresenter::TimetablePresenter;
use chrono::NaiveDateTime;

pub struct TimetablePresenterPlainText {
    result: String,
}

impl TimetablePresenterPlainText {}

impl TimetablePresenterPlainText {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { result: "".into() }
    }

    fn add_line(&mut self, line: &str) {
        self.result.push_str(line);
        self.result.push_str("\n")
    }

    fn get_timetablestop(&mut self, timetable: &Timetable) {
        if let Some(s) = &timetable.s {
            for item in s.iter() {
                self.get_departure(item);
            }
        } else {
            self.add_line("Timetable is empty.");
        }
    }

    fn get_departure(&mut self, s: &crate::timetable::TimetableStop) {
        if let Some(dp) = &s.dp {
            if let Some(tl) = &s.tl {
                self.get_train_info(tl, dp);
                self.get_time_info(dp);
                self.add_seperator_lines(1);
            }
        }
    }

    fn get_time_info(&mut self, dp: &ArrivalDeparture) {
        self.get_planned_time(dp);
        self.get_changed_time(dp);
    }

    fn get_planned_time(&mut self, dp: &ArrivalDeparture) {
        let pt = dp.pt.as_deref().unwrap_or("-");
        if let Ok(dt) = NaiveDateTime::parse_from_str(pt, "%y%m%d%H%M") {
            self.add_line(&format!("Planned time: {}", dt));
        }
    }

    fn get_changed_time(&mut self, dp: &crate::timetable::ArrivalDeparture) {
        let ct = dp.ct.as_deref().unwrap_or("-");
        if let Ok(dt) = NaiveDateTime::parse_from_str(ct, "%y%m%d%H%M") {
            self.add_line(&format!("Actual time: {}", dt));
        } else {
            self.add_line("Actual time: No delay");
        }
    }

    fn get_train_info(
        &mut self,
        tl: &crate::timetable::Triplabel,
        dp: &crate::timetable::ArrivalDeparture,
    ) {
        self.add_line(&format!(
            "{} {}: {}",
            tl.c.as_ref().unwrap_or(&"".to_string()),
            dp.l.as_ref().unwrap_or(&"".to_string()),
            dp.ppth
                .as_ref()
                .unwrap_or(&"".to_string())
                .split('|')
                .last()
                .unwrap_or(""),
        ));
    }

    fn get_station_name(&mut self, timetable: &Timetable) {
        self.add_line(&format!(
            "{}",
            timetable
                .station
                .as_ref()
                .unwrap_or(&"Station name missing".to_string())
        ));
    }

    fn add_seperator_lines(&mut self, count: i16) {
        for _n in 0..count {
            self.add_line("----------------------");
        }
    }
}

impl TimetablePresenter for TimetablePresenterPlainText {
    fn present(&mut self, timetable: &Timetable) -> Result<String, String> {
        self.get_station_name(timetable);
        self.add_seperator_lines(2);
        self.get_timetablestop(timetable);
        self.add_seperator_lines(1);

        match self.result.is_empty() {
            true => Err("Plaintext conversion error".to_string()),
            false => Ok(self.result.clone()),
        }
    }
}
