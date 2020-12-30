use covid_service_rs::data_processing;
use covid_service_rs::schema::TimeSeriesCase;

fn vec_compare(v1: Vec<TimeSeriesCase>, v2: Vec<TimeSeriesCase>) -> bool {
    (v1.len() == v2.len()) && v1.iter().zip(v2).all(|(a, b)| a == &b)
}

#[test]
fn string_hyphenation() {
    assert_eq!(
        data_processing::hyphenate_string("hello world".to_string()),
        "hello-world".to_string()
    );
}

#[test]
fn empty_string_hyphenation() {
    assert_eq!(
        data_processing::hyphenate_string("".to_string()),
        "".to_string()
    );
}

#[test]
fn multiple_worded_string_hyphenation() {
    assert_eq!(
        data_processing::hyphenate_string("Saint Vincent and the Grenadines".to_string()),
        "saint-vincent-and-the-grenadines".to_string()
    );
}

#[test]
fn worded_start_and_end_with_space_string_hyphenation() {
    assert_eq!(
        data_processing::hyphenate_string(" San Marino ".to_string()),
        "san-marino".to_string()
    );
}

#[test]
fn id_key_gen() {
    assert_eq!(
        data_processing::generate_id_key(&Some("Hong Kong".to_string()), &"China".to_string()),
        "china-hong-kong".to_string()
    );
    assert_ne!(
        data_processing::generate_id_key(&Some("Hong Kong".to_string()), &"China".to_string()),
        "china hong kong".to_string()
    );
}

#[test]
fn id_key_gen_no_province() {
    assert_eq!(
        data_processing::generate_id_key(&None, &"China".to_string()),
        "china".to_string()
    );
}

#[test]
fn id_key_gen_no_country() {
    assert_eq!(
        data_processing::generate_id_key(&Some("Hong Kong".to_string()), &"".to_string()),
        "".to_string()
    );
}

#[test]
fn id_key_gen_with_quote_mark() {
    assert_eq!(
        data_processing::generate_id_key(&None, &"Cote d'Ivoire".to_string()),
        "cote-divoire".to_string()
    );
}

#[test]
fn id_key_gen_with_bracket() {
    assert_eq!(
        data_processing::generate_id_key(&None, &"Congo (Brazzaville)".to_string()),
        "congo-brazzaville".to_string()
    );
}

#[test]
fn id_key_gen_with_comma() {
    assert_eq!(
        data_processing::generate_id_key(
            &Some("Bonaire, Sint Eustatius and Saba".to_string()),
            &"Netherlands".to_string()
        ),
        "netherlands-bonaire-sint-eustatius-and-saba".to_string()
    );
}

#[test]
fn join_two_time_series_vectors_with_one_entry() {
    let day1_province1 = TimeSeriesCase {
        confirmed: 20,
        deaths: 5,
        confirmedCasesToday: 3,
        deathsToday: 4,
        day: "02/12/20".to_string(),
    };
    let day1_province2 = TimeSeriesCase {
        confirmed: 560,
        deaths: 30,
        confirmedCasesToday: 56,
        deathsToday: 33,
        day: "02/12/20".to_string(),
    };
    let combined_day1 = TimeSeriesCase {
        confirmed: 580,
        deaths: 35,
        confirmedCasesToday: 59,
        deathsToday: 37,
        day: "02/12/20".to_string(),
    };
    let vec1 = Vec::from([day1_province1]);
    let vec2 = Vec::from([day1_province2]);
    let result = Vec::from([combined_day1]);
    assert!(vec_compare(
        data_processing::combine_time_series_cases(vec1, vec2),
        result
    ));
}

#[test]
fn join_two_time_series_vectors_with_two_entries() {
    let day1_province1 = TimeSeriesCase {
        confirmed: 20,
        deaths: 5,
        confirmedCasesToday: 3,
        deathsToday: 4,
        day: "02/12/20".to_string(),
    };
    let day2_province1 = TimeSeriesCase {
        confirmed: 27,
        deaths: 8,
        confirmedCasesToday: 7,
        deathsToday: 3,
        day: "03/12/20".to_string(),
    };
    let day1_province2 = TimeSeriesCase {
        confirmed: 560,
        deaths: 30,
        confirmedCasesToday: 56,
        deathsToday: 33,
        day: "02/12/20".to_string(),
    };
    let day2_province2 = TimeSeriesCase {
        confirmed: 569,
        deaths: 31,
        confirmedCasesToday: 9,
        deathsToday: 1,
        day: "03/12/20".to_string(),
    };
    let combined_day1 = TimeSeriesCase {
        confirmed: 580,
        deaths: 35,
        confirmedCasesToday: 59,
        deathsToday: 37,
        day: "02/12/20".to_string(),
    };
    let combined_day2 = TimeSeriesCase {
        confirmed: 596,
        deaths: 39,
        confirmedCasesToday: 16,
        deathsToday: 4,
        day: "03/12/20".to_string(),
    };
    let vec1 = Vec::from([day1_province1, day2_province1]);
    let vec2 = Vec::from([day1_province2, day2_province2]);
    let result = Vec::from([combined_day1, combined_day2]);
    assert!(vec_compare(
        data_processing::combine_time_series_cases(vec1, vec2),
        result
    ));
}
