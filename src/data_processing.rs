use std::collections::BTreeMap;
use std::error::Error;

use crate::schema::{Case, CaseByLocation, CsvCase, TimeSeriesCase};

fn hyphenate_string(s: String) -> String {
    s.replace(' ', "-").to_lowercase()
}

fn generate_id_key(province: &Option<String>, country: &String) -> String {
    let country = hyphenate_string(country.to_string());
    if let Some(province) = province {
        let province = hyphenate_string(province.to_string());
        format!("{}-{}", country, province)
    } else {
        country
    }
}

pub fn merge_csv_gis_cases(csv_cases: Vec<CsvCase>, gis_cases: Vec<Case>) -> Vec<CaseByLocation> {
    // TODO: implement merge
    Vec::new()
}

pub fn process_csv(
    confirmed: String,
    deaths: String,
) -> Result<(Vec<CsvCase>, BTreeMap<usize, TimeSeriesCase>), Box<dyn Error>> {
    let mut cases = Vec::new();
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
        let _id_key = generate_id_key(&province, &country);
        cases.push(CsvCase {
            Province_State: province,
            Country_Region: country,
            Lat: confirmed_record[2].parse().unwrap_or_default(),
            Long_: confirmed_record[3].parse().unwrap_or_default(),
            cases: time_series,
        });
    }
    println!("Global cases date map keys: {:?}", global_cases_map.len());
    Ok((cases, global_cases_map))
}
