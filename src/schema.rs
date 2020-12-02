use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GlobalCaseByLocation {
    pub confirmed: i64,
    pub active: i64,
    pub recovered: i64,
    pub deaths: i64,
    pub confirmedCasesToday: i64,
    pub deathsToday: i64,
    pub timeSeriesTotalCasesByDate: Vec<TimeSeriesCase>,
    pub timeStamp: bson::DateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CaseByLocation {
    pub idKey: String,
    pub country: String,
    pub province: Option<String>,
    pub countryCode: String,
    pub confirmed: i64,
    pub deaths: i64,
    pub recovered: i64,
    pub active: i64,
    pub confirmedCasesToday: i64,
    pub deathsToday: i64,
    pub lastUpdate: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub casesByDate: Vec<TimeSeriesCase>,
    pub provincesList: Vec<Province>,
    pub hasProvince: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Province {
    idKey: String,
    province: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TimeSeriesCase {
    pub confirmed: i64,
    pub deaths: i64,
    pub confirmedToday: i64,
    pub deathsToday: i64,
    pub day: String,
}

#[allow(non_snake_case)]
impl TimeSeriesCase {
    pub fn new(
        confirmed: i64,
        deaths: i64,
        confirmedToday: i64,
        deathsToday: i64,
        day: String,
    ) -> TimeSeriesCase {
        TimeSeriesCase {
            confirmed: confirmed,
            deaths: deaths,
            confirmedToday: confirmedToday,
            deathsToday: deathsToday,
            day: day,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Case {
    pub Province_State: Option<String>,
    pub Country_Region: String,
    pub Last_Update: Option<i64>,
    pub Lat: Option<f64>,
    pub Long_: Option<f64>,
    pub Confirmed: i64,
    pub Recovered: i64,
    pub Deaths: i64,
    pub Active: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CsvCase {
    pub Province_State: Option<String>,
    pub Country_Region: String,
    pub Lat: f64,
    pub Long_: f64,
    pub cases: Vec<TimeSeriesCase>,
}

#[derive(Deserialize, Debug)]
pub struct TotalsValue {
    pub value: i64,
}

#[derive(Deserialize, Debug)]
pub struct TotalsAttributes {
    pub attributes: TotalsValue,
}

#[derive(Deserialize, Debug)]
pub struct Total {
    pub features: Vec<TotalsAttributes>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CasesAttributes {
    pub attributes: Case,
}

#[derive(Deserialize, Debug)]
pub struct CasesByCountry {
    pub features: Vec<CasesAttributes>,
}
