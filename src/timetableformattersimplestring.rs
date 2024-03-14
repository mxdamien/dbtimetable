use crate::timetable::{ArrivalDeparture, Timetable};
use crate::timetableformatter::TimetableFormatter;
use chrono::NaiveDateTime;

pub struct TimetableFormatterSimpleString();

impl TimetableFormatterSimpleString {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl TimetableFormatter for TimetableFormatterSimpleString {
    fn format(&self, timetable: &Timetable) -> Result<String, String> {
        let mut ret: String = "".to_string();
        ret.push_str(&station_name(timetable));
        ret.push_str(&seperator_lines(2));
        ret.push_str(&timetablestop(timetable));
        ret.push_str(&seperator_lines(1));

        Ok(ret)
    }
}

fn timetablestop(timetable: &Timetable) -> String {
    if let Some(s) = &timetable.s {
        let mut ret: String = "".to_string();
        for item in s.iter() {
            ret.push_str(&print_departure(item))
        }
        ret
    } else {
        "Timetable is empty.\n".to_string()
    }
}

fn print_departure(s: &crate::timetable::TimetableStop) -> String {
    let mut ret: String = "".to_string();
    if let Some(dp) = &s.dp {
        if let Some(tl) = &s.tl {
            ret.push_str(&train_info(tl, dp));
            ret.push_str(&time_info(dp));
            ret.push_str(&seperator_lines(1));
        }
    }
    ret
}

fn time_info(dp: &ArrivalDeparture) -> String {
    let mut ret: String = "".to_string();
    ret.push_str(&planned_time(dp));
    ret.push_str(&changed_time(dp));
    ret
}

fn planned_time(dp: &ArrivalDeparture) -> String {
    let pt = dp.pt.as_deref().unwrap_or("-");
    if let Ok(dt) = NaiveDateTime::parse_from_str(pt, "%y%m%d%H%M") {
        return format!("Planned time: {}\n", dt);
    }
    "".to_string()
}

fn changed_time(dp: &crate::timetable::ArrivalDeparture) -> String {
    let ct: &str = dp.ct.as_deref().unwrap_or("-");
    match NaiveDateTime::parse_from_str(ct, "%y%m%d%H%M") {
        Ok(dt) => format!("Actual time: {}\n", dt),
        Err(_) => "Actual time: No delay\n".to_string(),
    }
}

fn train_info(tl: &crate::timetable::Triplabel, dp: &crate::timetable::ArrivalDeparture) -> String {
    format!(
        "{} {}: {}\n",
        tl.c.as_ref().unwrap_or(&"".to_string()),
        dp.l.as_ref().unwrap_or(&"".to_string()),
        dp.ppth
            .as_ref()
            .unwrap_or(&"".to_string())
            .split('|')
            .last()
            .unwrap_or("")
    )
}

fn station_name(timetable: &Timetable) -> String {
    format!(
        "{}\n",
        timetable
            .station
            .as_ref()
            .unwrap_or(&"Station name missing\n".to_string())
    )
}

fn seperator_lines(count: i16) -> String {
    let mut ret: String = "".to_string();
    for _n in 0..count {
        ret.push_str("----------------------\n");
    }
    ret
}
