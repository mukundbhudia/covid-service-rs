use covid_service_rs::data_processing;
use covid_service_rs::schema::{CsvCase, Region, TimeSeriesCase};
use std::collections::{BTreeMap, HashMap};

#[test]
fn basic_process_1_country_4_days_global_csv() {
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

    let (actual_global_cases, actual_time_series_map) = data_processing::process_csv(
        global_confirmed_csv,
        global_deaths_csv,
        Region::Global,
        today,
        Some(global_confirmed_and_deaths_count),
    )
    .unwrap();

    assert_eq!(expected_global_cases, actual_global_cases);
    assert_eq!(expected_time_series_map, actual_time_series_map);
}

#[test]
fn basic_process_1_country_4_days_8_confirmed_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,5,5,6"
            .to_string();
    let global_deaths_csv = "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,0,0"
        .to_string();

    let global_confirmed_and_deaths_count = (8, 0);

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
            confirmed: 5,
            deaths: 0,
            confirmedCasesToday: 5,
            deathsToday: 0,
            day: "1/23/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 5,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 6,
            deaths: 0,
            confirmedCasesToday: 1,
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
        confirmed: 8,
        deaths: 0,
        confirmedCasesToday: 2,
        deathsToday: 0,
        day: today.clone(),
    });

    let first_day_csv_index = 4;
    let expected_time_series_map = (first_day_csv_index..)
        .zip(expected_afghanistan_time_series_cases)
        .collect::<BTreeMap<usize, TimeSeriesCase>>();

    let (actual_global_cases, actual_time_series_map) = data_processing::process_csv(
        global_confirmed_csv,
        global_deaths_csv,
        Region::Global,
        today,
        Some(global_confirmed_and_deaths_count),
    )
    .unwrap();

    assert_eq!(expected_global_cases, actual_global_cases);
    assert_eq!(expected_time_series_map, actual_time_series_map);
}

#[test]
fn basic_process_1_country_4_days_7_deaths_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,0,0"
            .to_string();
    let global_deaths_csv = "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,1,1"
        .to_string();

    let global_confirmed_and_deaths_count = (0, 7);

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
            deaths: 1,
            confirmedCasesToday: 0,
            deathsToday: 1,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 0,
            deaths: 1,
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
        deaths: 7,
        confirmedCasesToday: 0,
        deathsToday: 6,
        day: today.clone(),
    });

    let first_day_csv_index = 4;
    let expected_time_series_map = (first_day_csv_index..)
        .zip(expected_afghanistan_time_series_cases)
        .collect::<BTreeMap<usize, TimeSeriesCase>>();

    let (actual_global_cases, actual_time_series_map) = data_processing::process_csv(
        global_confirmed_csv,
        global_deaths_csv,
        Region::Global,
        today,
        Some(global_confirmed_and_deaths_count),
    )
    .unwrap();

    assert_eq!(expected_global_cases, actual_global_cases);
    assert_eq!(expected_time_series_map, actual_time_series_map);
}

#[test]
fn basic_process_1_country_4_days_8_confirmed_7_deaths_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,5,5,6"
            .to_string();
    let global_deaths_csv = "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,1,1"
        .to_string();

    let global_confirmed_and_deaths_count = (8, 7);

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
            confirmed: 5,
            deaths: 0,
            confirmedCasesToday: 5,
            deathsToday: 0,
            day: "1/23/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 5,
            deaths: 1,
            confirmedCasesToday: 0,
            deathsToday: 1,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 6,
            deaths: 1,
            confirmedCasesToday: 1,
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
        confirmed: 8,
        deaths: 7,
        confirmedCasesToday: 2,
        deathsToday: 6,
        day: today.clone(),
    });

    let first_day_csv_index = 4;
    let expected_time_series_map = (first_day_csv_index..)
        .zip(expected_afghanistan_time_series_cases)
        .collect::<BTreeMap<usize, TimeSeriesCase>>();

    let (actual_global_cases, actual_time_series_map) = data_processing::process_csv(
        global_confirmed_csv,
        global_deaths_csv,
        Region::Global,
        today,
        Some(global_confirmed_and_deaths_count),
    )
    .unwrap();

    assert_eq!(expected_global_cases, actual_global_cases);
    assert_eq!(expected_time_series_map, actual_time_series_map);
}
