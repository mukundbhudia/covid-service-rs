use chrono::prelude::*;
use chrono::Utc;
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

// If the first day recorded has non-zero cases the calculated cases for 'today' should also be non-zero/
// For example if the first day on record has 2 confirmed cases, then `confirmedCasesToday` should be 2.
#[test]
fn non_zero_first_day_case_1_country_4_days_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,2,5,5,6"
            .to_string();
    let global_deaths_csv = "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,1,1,1,3"
        .to_string();

    let global_confirmed_and_deaths_count = (7, 4);

    let today = String::from("1/26/20");

    let mut expected_afghanistan_time_series_cases = vec![
        TimeSeriesCase {
            confirmed: 2,
            deaths: 1,
            confirmedCasesToday: 2,
            deathsToday: 1,
            day: "1/22/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 5,
            deaths: 1,
            confirmedCasesToday: 3,
            deathsToday: 0,
            day: "1/23/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 5,
            deaths: 1,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 6,
            deaths: 3,
            confirmedCasesToday: 1,
            deathsToday: 2,
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
        confirmed: 7,
        deaths: 4,
        confirmedCasesToday: 1,
        deathsToday: 1,
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

// Sometimes the current count of cases from the source is lower than the previous days,
// in this case we want today's count in the time series to be 0 instead of negative.
#[test]
fn no_negative_confirmed_today_process_1_country_4_days_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,5,8,9"
            .to_string();
    let global_deaths_csv = "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,1,1"
        .to_string();

    let global_confirmed_and_deaths_count = (6, 0);

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
            confirmed: 8,
            deaths: 1,
            confirmedCasesToday: 3,
            deathsToday: 1,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 9,
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
        confirmed: 6,
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
fn over_new_year_process_1_country_4_days_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,12/29/20,12/30/20,12/31/20,1/1/21
,Afghanistan,33.93911,67.709953,0,0,0,0"
            .to_string();
    let global_deaths_csv =
        "Province/State,Country/Region,Lat,Long,12/29/20,12/30/20,12/31/20,1/1/21
,Afghanistan,33.93911,67.709953,0,0,0,0"
            .to_string();

    let global_confirmed_and_deaths_count = (0, 0);
    //Using the same time `.format()` as in main
    let today = Utc.ymd(2021, 1, 2).format("%-m/%-d/%-y").to_string();

    let mut expected_afghanistan_time_series_cases = vec![
        TimeSeriesCase {
            confirmed: 0,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "12/29/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 0,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "12/30/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 0,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "12/31/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 0,
            deaths: 0,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "1/1/21".to_string(),
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
        day: "1/2/21".to_string(),
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
