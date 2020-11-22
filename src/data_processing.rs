use std::collections::{BTreeMap, HashMap};
use std::error::Error;

use crate::alpha3_country_codes::alpha_codes;
use crate::schema::{Case, CaseByLocation, CsvCase, TimeSeriesCase};

fn hyphenate_string(s: String) -> String {
    s.replace(' ', "-").to_lowercase()
}

pub fn generate_id_key(province: &Option<String>, country: &String) -> String {
    country.to_lowercase();
    let country = hyphenate_string(country.to_string());
    if let Some(province) = province {
        if province == "Unknown" {
            country
        } else {
            let province = hyphenate_string(province.to_string());
            format!("{}-{}", country, province)
        }
    } else {
        country
    }
}

pub fn merge_csv_gis_cases(
    mut csv_cases: HashMap<String, CsvCase>,
    mut gis_cases: HashMap<String, Case>,
) -> Vec<CaseByLocation> {
    let mut cases_by_location = Vec::new();
    let alpha_codes = alpha_codes();

    for (id_key, csv_case) in csv_cases.drain() {
        // println!("idkey: {}, province: {:?}", id_key, csv_case.Province_State);
        // let gis_case = gis_cases.get_mut(&id_key).unwrap();
        if let Some(gis_case) = gis_cases.get_mut(&id_key) {
            let country_code = match alpha_codes.get(&csv_case.Country_Region) {
                Some(code) => code.to_string(),
                None => String::new(),
            };
            cases_by_location.push(CaseByLocation {
                idKey: id_key,
                countryCode: country_code,
                active: gis_case.Active,
                confirmed: gis_case.Confirmed,
                recovered: gis_case.Recovered,
                country: csv_case.Country_Region,
                deaths: gis_case.Deaths,
                confirmedCasesToday: gis_case.Confirmed - csv_case.cases.last().unwrap().confirmed,
                deathsToday: gis_case.Deaths - csv_case.cases.last().unwrap().deaths,
                lastUpdate: gis_case.Last_Update,
                latitude: csv_case.Lat,
                longitude: csv_case.Long_,
                hasProvince: match &csv_case.Province_State {
                    None => false,
                    Some(_) => true,
                },
                province: csv_case.Province_State,
                casesByDate: csv_case.cases,
                provincesList: Vec::new(), // TODO: Form value here
            });
        }
    }
    cases_by_location
}

pub fn process_csv(
    confirmed: String,
    deaths: String,
) -> Result<(HashMap<String, CsvCase>, BTreeMap<usize, TimeSeriesCase>), Box<dyn Error>> {
    let mut cases = HashMap::new();
    let mut global_cases_map: BTreeMap<usize, TimeSeriesCase> = BTreeMap::new();
    let mut confirmed_csv_reader = csv::Reader::from_reader(confirmed.as_bytes());
    let mut deaths_csv_reader = csv::Reader::from_reader(deaths.as_bytes());
    let csv_headers = confirmed_csv_reader
        .headers()?
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for (confirmed_record, deaths_record) in confirmed_csv_reader
        .records()
        .zip(deaths_csv_reader.records())
    {
        let confirmed_record = confirmed_record?;
        let deaths_record = deaths_record?;
        let mut time_series: Vec<TimeSeriesCase> = Vec::new();
        let first_day_index = 4;
        let mut confirmed_today = 0;
        let mut deaths_today = 0;
        for i in first_day_index..confirmed_record.len() {
            let confirmed_cases = confirmed_record[i].parse::<i64>().unwrap_or_default();
            let confirmed_cases_yesterday =
                confirmed_record[i - 1].parse::<i64>().unwrap_or_default();
            let death_cases = deaths_record[i].parse::<i64>().unwrap_or_default();
            let death_cases_yesterday = deaths_record[i - 1].parse::<i64>().unwrap_or_default();

            if i != first_day_index {
                confirmed_today = confirmed_cases - confirmed_cases_yesterday;
                deaths_today = death_cases - death_cases_yesterday;
            }
            let day = &csv_headers[i];
            let time_series_case = TimeSeriesCase::new(
                confirmed_cases,
                death_cases,
                confirmed_today,
                deaths_today,
                day.to_string(),
            );

            let ts_case_to_change = global_cases_map.entry(i).or_insert(time_series_case);
            ts_case_to_change.confirmed += confirmed_cases;
            ts_case_to_change.deaths += death_cases;
            ts_case_to_change.confirmedToday += confirmed_today;
            ts_case_to_change.deathsToday += deaths_today;

            time_series.push(TimeSeriesCase::new(
                confirmed_cases,
                death_cases,
                confirmed_today,
                deaths_today,
                day.to_string(),
            ));
        }
        let province = match confirmed_record[0].is_empty() {
            true => None,
            false => Some(confirmed_record[0].to_string()),
        };
        let country = confirmed_record[1].to_string();
        let id_key = generate_id_key(&province, &country);
        cases.insert(
            id_key,
            CsvCase {
                Province_State: province,
                Country_Region: country,
                Lat: confirmed_record[2].parse().unwrap_or_default(),
                Long_: confirmed_record[3].parse().unwrap_or_default(),
                cases: time_series,
            },
        );
    }
    println!("Global cases date map keys: {:?}", global_cases_map.len());
    Ok((cases, global_cases_map))
}
