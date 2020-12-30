use covid_service_rs::data_processing;
use covid_service_rs::schema::{CsvCase, Region, TimeSeriesCase};
use std::collections::{BTreeMap, HashMap};

#[test]
fn basic_process_one_country_4_days_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,0,0"
            .to_string();
    let global_deaths_csv = "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,0,0"
        .to_string();

    let global_confirmed_and_deaths_count = (0, 0);

    let today = String::from("1/26/20");

    let mut expected_afghanistan_time_series_cases = vec![
        TimeSeriesCase {
            confirmed: 0,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "1/22/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 0,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "1/23/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 0,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 0,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "1/25/20".to_string(),
        },
    ];

    let mut expected_global_cases = HashMap::new();
    expected_global_cases.insert(
        "afghanistan".to_string(),
        CsvCase {
            Province_State: None,
            Country_Region: "Afghanistan".to_string(),
            Lat: 33.93911,
            Long_: 67.709953,
            cases: expected_afghanistan_time_series_cases.clone(),
        },
    );

    // We add the current day's cases to the expected time series
    expected_afghanistan_time_series_cases.push(TimeSeriesCase {
        confirmed: 0,
        deaths: 0,
        confirmedCasesToday: 0,
        deathsToday: 0,
        day: today.clone(),
    });

    let first_day_csv_index = 4;
    let expected_time_series_map = (first_day_csv_index..)
        .zip(expected_afghanistan_time_series_cases)
        .collect::<BTreeMap<usize, TimeSeriesCase>>();

    let result = data_processing::process_csv(
        global_confirmed_csv,
        global_deaths_csv,
        Region::Global,
        today,
        Some(global_confirmed_and_deaths_count),
    );

    let (actual_global_cases, actual_time_series_map) = result.unwrap();

    assert_eq!(expected_global_cases, actual_global_cases);
    assert_eq!(expected_time_series_map, actual_time_series_map);
}
