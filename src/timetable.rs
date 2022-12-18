use merge::Merge;
use serde::Deserialize;

#[derive(Deserialize, Merge)]
pub struct ArrivalDeparture {
    pub cde: Option<String>,
    pub clt: Option<String>,
    pub cp: Option<String>,
    pub cpth: Option<String>,
    pub cs: Option<String>,
    pub ct: Option<String>,
    pub dc: Option<i64>,
    pub hi: Option<i32>,
    pub l: Option<String>,
    pub m: Option<Vec<Message>>,
    pub pde: Option<String>,
    pub pp: Option<String>,
    pub ppth: Option<String>,
    pub ps: Option<String>,
    pub pt: Option<String>,
    pub tra: Option<String>,
    pub wings: Option<String>,
}

#[derive(Deserialize, Merge)]
pub struct TimetableStop {
    pub eva: Option<i64>,
    pub id: Option<String>,
    pub ar: Option<ArrivalDeparture>,
    pub conn: Option<String>,
    pub dp: Option<ArrivalDeparture>,
    pub hd: Option<String>,
    pub hpc: Option<String>,
    pub m: Option<Vec<Message>>,
    pub r: Option<String>,
    pub rtr: Option<String>,
    pub tl: Option<Triplabel>,
    pub f: Option<String>,
}

#[derive(Deserialize, Merge)]
pub struct Triplabel {
    pub c: Option<String>,
    pub n: Option<String>,
    pub o: Option<String>,
    pub f: Option<String>,

    #[serde(rename = "false")]
    pub fa: Option<String>,
    pub t: Option<String>,
}

#[derive(Deserialize, Merge)]
pub struct Message {
    pub id: Option<String>,
    pub t: Option<String>,
    pub ts: Option<String>,
    pub c: Option<String>,
    pub cat: Option<String>,
    pub del: Option<String>,
    pub dm: Option<String>,
    pub ec: Option<String>,
    pub elnk: Option<String>,
    pub ext: Option<String>,
    pub from: Option<String>,
    pub int: Option<String>,
    pub o: Option<String>,
    pub pr: Option<String>,
    pub tl: Option<Triplabel>,
    pub to: Option<String>,
}

#[derive(Deserialize, Merge)]
pub struct Timetable {
    pub station: Option<String>,
    pub eva: Option<i64>,
    pub m: Option<Vec<Message>>,
    pub s: Option<Vec<TimetableStop>>,
}
