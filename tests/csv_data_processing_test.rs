use chrono::prelude::*;
use chrono::Utc;
use covid_service_rs::data_processing;
use covid_service_rs::schema::{CsvCase, HighestCase, Region, TimeSeriesCase};
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

    let global_confirmed_and_deaths_count = (0, 0, 0);

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
            dateOfFirstCase: None,
            dateOfFirstDeath: None,
            highestDailyConfirmed: HighestCase {
                count: 0,
                date: None,
            },
            highestDailyDeaths: HighestCase {
                count: 0,
                date: None,
            },
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

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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

    let global_confirmed_and_deaths_count = (8, 0, 0);

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
            dateOfFirstCase: Some("1/23/20".to_string()),
            dateOfFirstDeath: None,
            highestDailyConfirmed: HighestCase {
                count: 5,
                date: Some("1/23/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 0,
                date: None,
            },
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

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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

    let global_confirmed_and_deaths_count = (0, 7, 0);

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
            dateOfFirstCase: None,
            dateOfFirstDeath: Some("1/24/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 0,
                date: None,
            },
            highestDailyDeaths: HighestCase {
                count: 1,
                date: Some("1/24/20".to_string()),
            },
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

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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

    let global_confirmed_and_deaths_count = (8, 7, 0);

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
            dateOfFirstCase: Some("1/23/20".to_string()),
            dateOfFirstDeath: Some("1/24/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 5,
                date: Some("1/23/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 1,
                date: Some("1/24/20".to_string()),
            },
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

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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

    let global_confirmed_and_deaths_count = (7, 4, 0);

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
            dateOfFirstCase: Some("1/22/20".to_string()),
            dateOfFirstDeath: Some("1/22/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 3,
                date: Some("1/23/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 2,
                date: Some("1/25/20".to_string()),
            },
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

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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

    let global_confirmed_and_deaths_count = (6, 0, 0);

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
            dateOfFirstCase: Some("1/23/20".to_string()),
            dateOfFirstDeath: Some("1/24/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 5,
                date: Some("1/23/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 1,
                date: Some("1/24/20".to_string()),
            },
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

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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

    let global_confirmed_and_deaths_count = (0, 0, 0);
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
            dateOfFirstCase: None,
            dateOfFirstDeath: None,
            highestDailyConfirmed: HighestCase {
                count: 0,
                date: None,
            },
            highestDailyDeaths: HighestCase {
                count: 0,
                date: None,
            },
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

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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
fn basic_process_2_countries_4_days_12_confirmed_10_deaths_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,5,5,6
,Congo (Brazzaville),-0.228,15.8277,0,1,4,4"
            .to_string();
    let global_deaths_csv = "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,1,1
,Congo (Brazzaville),-0.228,15.8277,1,2,5,5"
        .to_string();

    let global_confirmed_and_deaths_count = (12, 10, 0);

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

    let mut expected_congo_brazzaville_time_series_cases = vec![
        TimeSeriesCase {
            confirmed: 0,
            deaths: 1,
            confirmedCasesToday: 0,
            deathsToday: 1,
            day: "1/22/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 1,
            deaths: 2,
            confirmedCasesToday: 1,
            deathsToday: 1,
            day: "1/23/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 4,
            deaths: 5,
            confirmedCasesToday: 3,
            deathsToday: 3,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 4,
            deaths: 5,
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
            dateOfFirstCase: Some("1/23/20".to_string()),
            dateOfFirstDeath: Some("1/24/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 5,
                date: Some("1/23/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 1,
                date: Some("1/24/20".to_string()),
            },
        },
    );
    expected_global_cases.insert(
        "congo".to_string(),
        CsvCase {
            Province_State: None,
            Country_Region: "Congo".to_string(),
            Lat: -0.228,
            Long_: 15.8277,
            cases: expected_congo_brazzaville_time_series_cases.clone(),
            dateOfFirstCase: Some("1/23/20".to_string()),
            dateOfFirstDeath: Some("1/22/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 3,
                date: Some("1/24/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 3,
                date: Some("1/24/20".to_string()),
            },
        },
    );

    // We add the current day's cases to the expected time series
    expected_afghanistan_time_series_cases.push(TimeSeriesCase {
        confirmed: 6,
        deaths: 5,
        confirmedCasesToday: 0,
        deathsToday: 3,
        day: today.clone(),
    });
    expected_congo_brazzaville_time_series_cases.push(TimeSeriesCase {
        confirmed: 6,
        deaths: 5,
        confirmedCasesToday: 2,
        deathsToday: 1,
        day: today.clone(),
    });

    let first_day_csv_index = 4;
    let joined_expected_time_series_map = expected_afghanistan_time_series_cases
        .into_iter()
        .zip(expected_congo_brazzaville_time_series_cases)
        .map(|(a, b)| TimeSeriesCase {
            confirmed: a.confirmed + b.confirmed,
            deaths: a.deaths + b.deaths,
            confirmedCasesToday: a.confirmedCasesToday + b.confirmedCasesToday,
            deathsToday: a.deathsToday + b.deathsToday,
            day: a.day.clone(),
        })
        .collect::<Vec<TimeSeriesCase>>();
    let expected_time_series_map = (first_day_csv_index..)
        .zip(joined_expected_time_series_map)
        .collect::<BTreeMap<usize, TimeSeriesCase>>();

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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
fn basic_process_2_countries_1_province_4_days_14_confirmed_12_deaths_global_csv() {
    let global_confirmed_csv =
        "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,5,5,6
,Congo (Brazzaville),-0.228,15.8277,0,1,4,4
Falkland Islands (Malvinas),United Kingdom,-51.7963,-59.5236,0,0,2,2"
            .to_string();
    let global_deaths_csv = "Province/State,Country/Region,Lat,Long,1/22/20,1/23/20,1/24/20,1/25/20
,Afghanistan,33.93911,67.709953,0,0,1,1
,Congo (Brazzaville),-0.228,15.8277,1,2,5,5
Falkland Islands (Malvinas),United Kingdom,-51.7963,-59.5236,0,0,1,2"
        .to_string();

    let global_confirmed_and_deaths_count = (14, 12, 0);

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

    let mut expected_congo_brazzaville_time_series_cases = vec![
        TimeSeriesCase {
            confirmed: 0,
            deaths: 1,
            confirmedCasesToday: 0,
            deathsToday: 1,
            day: "1/22/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 1,
            deaths: 2,
            confirmedCasesToday: 1,
            deathsToday: 1,
            day: "1/23/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 4,
            deaths: 5,
            confirmedCasesToday: 3,
            deathsToday: 3,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 4,
            deaths: 5,
            confirmedCasesToday: 0,
            deathsToday: 0,
            day: "1/25/20".to_string(),
        },
    ];

    let mut expected_uk_falkland_islands_time_series_cases = vec![
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
            confirmed: 2,
            deaths: 1,
            confirmedCasesToday: 2,
            deathsToday: 1,
            day: "1/24/20".to_string(),
        },
        TimeSeriesCase {
            confirmed: 2,
            deaths: 2,
            confirmedCasesToday: 0,
            deathsToday: 1,
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
            dateOfFirstCase: Some("1/23/20".to_string()),
            dateOfFirstDeath: Some("1/24/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 5,
                date: Some("1/23/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 1,
                date: Some("1/24/20".to_string()),
            },
        },
    );
    expected_global_cases.insert(
        "congo".to_string(),
        CsvCase {
            Province_State: None,
            Country_Region: "Congo".to_string(),
            Lat: -0.228,
            Long_: 15.8277,
            cases: expected_congo_brazzaville_time_series_cases.clone(),
            dateOfFirstCase: Some("1/23/20".to_string()),
            dateOfFirstDeath: Some("1/22/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 3,
                date: Some("1/24/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 3,
                date: Some("1/24/20".to_string()),
            },
        },
    );
    expected_global_cases.insert(
        "united-kingdom-falkland-islands-malvinas".to_string(),
        CsvCase {
            Province_State: Some("Falkland Islands (Malvinas)".to_string()),
            Country_Region: "United Kingdom".to_string(),
            Lat: -51.7963,
            Long_: -59.5236,
            cases: expected_uk_falkland_islands_time_series_cases.clone(),
            dateOfFirstCase: Some("1/24/20".to_string()),
            dateOfFirstDeath: Some("1/24/20".to_string()),
            highestDailyConfirmed: HighestCase {
                count: 2,
                date: Some("1/24/20".to_string()),
            },
            highestDailyDeaths: HighestCase {
                count: 1,
                date: Some("1/24/20".to_string()),
            },
        },
    );

    // We add the current day's cases to the expected time series
    expected_afghanistan_time_series_cases.push(TimeSeriesCase {
        confirmed: 6,
        deaths: 5,
        confirmedCasesToday: 0,
        deathsToday: 3,
        day: today.clone(),
    });
    expected_congo_brazzaville_time_series_cases.push(TimeSeriesCase {
        confirmed: 6,
        deaths: 5,
        confirmedCasesToday: 2,
        deathsToday: 1,
        day: today.clone(),
    });
    expected_uk_falkland_islands_time_series_cases.push(TimeSeriesCase {
        confirmed: 2,
        deaths: 2,
        confirmedCasesToday: 0,
        deathsToday: 0,
        day: today.clone(),
    });

    let first_day_csv_index = 4;
    let joined_expected_time_series_map = expected_afghanistan_time_series_cases
        .into_iter()
        .zip(expected_congo_brazzaville_time_series_cases.into_iter())
        .zip(expected_uk_falkland_islands_time_series_cases.into_iter())
        .map(|((a, b), c)| TimeSeriesCase {
            confirmed: a.confirmed + b.confirmed + c.confirmed,
            deaths: a.deaths + b.deaths + c.deaths,
            confirmedCasesToday: a.confirmedCasesToday
                + b.confirmedCasesToday
                + c.confirmedCasesToday,
            deathsToday: a.deathsToday + b.deathsToday + c.deathsToday,
            day: a.day.clone(),
        })
        .collect::<Vec<TimeSeriesCase>>();
    let expected_time_series_map = (first_day_csv_index..)
        .zip(joined_expected_time_series_map)
        .collect::<BTreeMap<usize, TimeSeriesCase>>();

    let (actual_global_cases, actual_time_series_map, _) = data_processing::process_csv(
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
