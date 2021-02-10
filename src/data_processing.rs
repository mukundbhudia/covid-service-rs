use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;

use crate::alpha3_country_codes::alpha_codes;
use crate::schema::{
    Case, CaseByLocation, CsvCase, GlobalCaseByDate, GlobalDayCase, HighestCase, Province, Region,
    TimeSeriesCase,
};

fn force_to_zero_if_negative(number: i64) -> i64 {
    match number.is_negative() {
        true => 0,
        false => number,
    }
}

pub fn hyphenate_string(s: String) -> String {
    s.replace(
        &['(', ')', ',', '\'', '\"', '.', ';', ':', '\'', '*'][..],
        "",
    )
    .to_lowercase()
    .trim()
    .replace(' ', "-")
}

pub fn generate_id_key(province: &Option<String>, country: &String) -> String {
    country.to_lowercase();
    let country = hyphenate_string(country.to_string());
    if country.len() > 0 {
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
    } else {
        country
    }
}

pub fn combine_time_series_cases(
    vec1: Vec<TimeSeriesCase>,
    vec2: Vec<TimeSeriesCase>,
) -> Vec<TimeSeriesCase> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x, y)| TimeSeriesCase {
            confirmed: x.confirmed + y.confirmed,
            deaths: x.deaths + y.deaths,
            confirmedCasesToday: x.confirmedCasesToday + y.confirmedCasesToday,
            deathsToday: x.deathsToday + y.deathsToday,
            day: x.day.clone(),
        })
        .collect()
}

fn get_highest_confirmed_and_deaths(
    cases: Vec<TimeSeriesCase>,
) -> (HighestCase, HighestCase, Option<String>, Option<String>) {
    let max_daily_confirmed = cases.iter().max_by_key(|x| x.confirmedCasesToday).unwrap();
    let max_daily_deaths = cases.iter().max_by_key(|x| x.deathsToday).unwrap();
    let first_case = cases.iter().find(|x| x.confirmed > 0);
    let first_death = cases.iter().find(|x| x.deaths > 0);
    (
        HighestCase {
            count: max_daily_confirmed.confirmedCasesToday,
            date: Some(max_daily_confirmed.day.clone()),
        },
        HighestCase {
            count: max_daily_deaths.deathsToday,
            date: Some(max_daily_deaths.day.clone()),
        },
        match first_case {
            Some(ts_case) => Some(ts_case.day.clone()),
            None => None,
        },
        match first_death {
            Some(ts_case) => Some(ts_case.day.clone()),
            None => None,
        },
    )
}

fn patch_country_names(country_name: String) -> String {
    match country_name.as_str() {
        "Burma" => "Myanmar".to_string(),
        "Congo (Brazzaville)" => "Congo".to_string(),
        "Congo (Kinshasa)" => "Democratic Republic of Congo".to_string(),
        "Korea, South" => "South Korea".to_string(),
        "Taiwan*" => "Taiwan".to_string(),
        "US" => "United States".to_string(),
        _ => country_name,
    }
}

pub fn merge_csv_gis_cases(
    mut csv_cases: HashMap<String, CsvCase>,
    mut gis_cases: HashMap<String, Case>,
    today: String,
) -> HashMap<String, CaseByLocation> {
    let mut cases_by_location: HashMap<String, CaseByLocation> = HashMap::new();
    let mut countries_with_provinces: HashMap<String, CaseByLocation> = HashMap::new();
    let alpha_codes = alpha_codes();

    for (id_key, mut csv_case) in csv_cases.drain() {
        if let Some(gis_case) = gis_cases.get_mut(&id_key) {
            let confirmed_cases_today = force_to_zero_if_negative(
                gis_case.Confirmed - csv_case.cases.last().unwrap().confirmed,
            );
            let deaths_today =
                force_to_zero_if_negative(gis_case.Deaths - csv_case.cases.last().unwrap().deaths);

            let today_time_series_cases = TimeSeriesCase::new(
                gis_case.Confirmed,
                gis_case.Deaths,
                confirmed_cases_today,
                deaths_today,
                today.clone(),
            );
            csv_case.cases.push(today_time_series_cases);

            let (
                highest_daily_confirmed,
                highest_daily_deaths,
                date_of_first_case,
                date_of_first_death,
            ) = get_highest_confirmed_and_deaths(csv_case.cases.clone());

            let (
                mut country_code,
                population,
                continent,
                population_density,
                median_age,
                aged_65_older,
                aged_70_older,
                gdp_per_capita,
                diabetes_prevalence,
                cardiovasc_death_rate,
                life_expectancy,
                human_development_index,
            ) = match alpha_codes.get(&csv_case.Country_Region) {
                Some(code) => (
                    code.iso_code.to_string(),
                    Some(code.population),
                    Some(code.continent.to_string()),
                    code.population_density,
                    code.median_age,
                    code.aged_65_older,
                    code.aged_70_older,
                    code.gdp_per_capita,
                    code.diabetes_prevalence,
                    code.cardiovasc_death_rate,
                    Some(code.life_expectancy),
                    code.human_development_index,
                ),
                None => (
                    "".to_string(),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
            };

            let confirmed_per_capita = match population {
                Some(pop) => Some(gis_case.Confirmed as f64 / pop as f64),
                None => None,
            };

            let deaths_per_capita = match population {
                Some(pop) => Some(gis_case.Deaths as f64 / pop as f64),
                None => None,
            };

            let province = csv_case.Province_State.clone();
            let id_key = generate_id_key(&province, &csv_case.Country_Region);
            if let Some(province_found) = &csv_case.Province_State {
                let province_type = Province {
                    idKey: id_key.clone(),
                    province: province_found.to_string(),
                };

                if province_found == "Greenland" {
                    country_code = match alpha_codes.get(&province_found.to_string()) {
                        Some(code) => code.iso_code.to_string(),
                        None => "".to_string(),
                    };
                }

                if let Some(case_found) = countries_with_provinces.get_mut(&csv_case.Country_Region) {
                    let combined_ts_cases = combine_time_series_cases(
                        case_found.casesByDate.clone(),
                        csv_case.cases.clone(),
                    );
                    let (
                        highest_daily_confirmed,
                        highest_daily_deaths,
                        date_of_first_case,
                        date_of_first_death,
                    ) = get_highest_confirmed_and_deaths(combined_ts_cases.clone());

                    case_found.confirmed += gis_case.Confirmed;
                    case_found.recovered += gis_case.Recovered;
                    case_found.active += gis_case.Active;
                    case_found.deaths += gis_case.Deaths;
                    case_found.confirmedCasesToday += confirmed_cases_today;
                    case_found.deathsToday += deaths_today;
                    case_found.casesByDate = combined_ts_cases.clone();
                    case_found.provincesList.push(province_type);
                    case_found.hasProvince = true;
                    case_found.dateOfFirstCase = date_of_first_case;
                    case_found.dateOfFirstDeath = date_of_first_death;
                    case_found.highestDailyConfirmed = highest_daily_confirmed;
                    case_found.highestDailyDeaths = highest_daily_deaths;
                } else {
                    countries_with_provinces.insert(
                        csv_case.Country_Region.clone(),
                        CaseByLocation {
                            idKey: generate_id_key(&None, &csv_case.Country_Region),
                            countryCode: country_code.clone(),
                            population,
                            active: gis_case.Active,
                            confirmed: gis_case.Confirmed,
                            confirmedPerCapita: confirmed_per_capita,
                            recovered: gis_case.Recovered,
                            country: csv_case.Country_Region.clone(),
                            deaths: gis_case.Deaths,
                            deathsPerCapita: deaths_per_capita,
                            confirmedCasesToday: confirmed_cases_today,
                            deathsToday: deaths_today,
                            lastUpdate: gis_case.Last_Update,
                            latitude: csv_case.Lat,
                            longitude: csv_case.Long_,
                            hasProvince: false,
                            province: None,
                            casesByDate: csv_case.cases.clone(),
                            provincesList: Vec::from([province_type]),
                            dateOfFirstCase: date_of_first_case.clone(),
                            dateOfFirstDeath: date_of_first_death.clone(),
                            highestDailyConfirmed: highest_daily_confirmed.clone(),
                            highestDailyDeaths: highest_daily_deaths.clone(),
                            continent: continent.clone(),
                            populationDensity: population_density,
                            medianAge: median_age,
                            aged65older: aged_65_older,
                            aged70older: aged_70_older,
                            gdpPerCapita: gdp_per_capita,
                            diabetesPrevalence: diabetes_prevalence,
                            cardiovascDeathRate: cardiovasc_death_rate,
                            lifeExpectancy: life_expectancy,
                            humanDevelopmentIndex: human_development_index,
                        },
                    );
                }
            }

            let has_province = match &csv_case.Province_State {
                None => countries_with_provinces.contains_key(&csv_case.Country_Region),
                Some(_) => false,
            };

            cases_by_location.insert(
                id_key.clone(),
                CaseByLocation {
                    idKey: id_key,
                    countryCode: country_code,
                    population: match province {
                        Some(_) => None,
                        None => population,
                    },
                    active: gis_case.Active,
                    confirmed: gis_case.Confirmed,
                    confirmedPerCapita: match province {
                        Some(_) => None,
                        None => confirmed_per_capita,
                    },
                    recovered: gis_case.Recovered,
                    country: csv_case.Country_Region,
                    deaths: gis_case.Deaths,
                    deathsPerCapita: match province {
                        Some(_) => None,
                        None => deaths_per_capita,
                    },
                    populationDensity: match province {
                        Some(_) => None,
                        None => population_density,
                    },
                    medianAge: match province {
                        Some(_) => None,
                        None => median_age,
                    },
                    aged65older: match province {
                        Some(_) => None,
                        None => aged_65_older,
                    },
                    aged70older: match province {
                        Some(_) => None,
                        None => aged_70_older,
                    },
                    gdpPerCapita: gdp_per_capita,
                    diabetesPrevalence: match province {
                        Some(_) => None,
                        None => diabetes_prevalence,
                    },
                    cardiovascDeathRate: match province {
                        Some(_) => None,
                        None => cardiovasc_death_rate,
                    },
                    lifeExpectancy: match province {
                        Some(_) => None,
                        None => life_expectancy,
                    },
                    humanDevelopmentIndex: human_development_index,
                    continent,
                    confirmedCasesToday: confirmed_cases_today,
                    deathsToday: deaths_today,
                    lastUpdate: gis_case.Last_Update,
                    latitude: csv_case.Lat,
                    longitude: csv_case.Long_,
                    hasProvince: has_province,
                    province,
                    casesByDate: csv_case.cases,
                    provincesList: Vec::new(),
                    dateOfFirstCase: date_of_first_case,
                    dateOfFirstDeath: date_of_first_death,
                    highestDailyConfirmed: highest_daily_confirmed,
                    highestDailyDeaths: highest_daily_deaths,
                },
            );
        }
    }
    cases_by_location.extend(countries_with_provinces);
    cases_by_location
}

pub fn process_cases_by_country(cases_by_country: Vec<Case>) -> HashMap<String, Case> {
    let mut cases_by_country_map: HashMap<String, Case> = HashMap::new();
    for case_by_country in cases_by_country {
        let country_name = patch_country_names(case_by_country.Country_Region.to_string());

        // These countries are fragmented, they are as one in CSVs but are
        // split by provinces in GIS cases. We join them up in this function
        let province_state = match country_name.as_str() {
            "Spain" => &None,
            "Brazil" => &None,
            "Belgium" => &None,
            "Russia" => &None,
            "Mexico" => &None,
            "Colombia" => &None,
            "Peru" => &None,
            "Chile" => &None,
            "Germany" => &None,
            "Italy" => &None,
            "Ukraine" => &None,
            "Japan" => &None,
            "Sweden" => &None,
            "India" => &None,
            "Pakistan" => &None,
            _ => &case_by_country.Province_State,
        };

        let mut id_key = generate_id_key(&province_state, &country_name);
        let mainland_id_key = generate_id_key(&Some("mainland".to_string()), &country_name);

        if let Some(province_name) = &case_by_country.Province_State {
            // UK is fragmented into different states in the GIS cases from its
            // mainland appearance in the CSV file. Below we assert them
            // to be part of the mainland
            if country_name.as_str() == "United Kingdom" {
                let province_name = province_name.as_str();
                id_key = match province_name {
                    "England" => mainland_id_key,
                    "Wales" => mainland_id_key,
                    "Scotland" => mainland_id_key,
                    "Northern Ireland" => mainland_id_key,
                    "Unknown" => mainland_id_key,
                    _ => id_key,
                }
            }
        } else {
            id_key = match country_name.as_str() {
                "France" => mainland_id_key,
                "Denmark" => mainland_id_key,
                "Netherlands" => mainland_id_key,
                _ => id_key,
            };
        }

        if let Some(case_found) = cases_by_country_map.get_mut(&id_key) {
            case_found.Confirmed += case_by_country.Confirmed;
            case_found.Recovered += case_by_country.Recovered;
            case_found.Active += case_by_country.Active;
            case_found.Deaths += case_by_country.Deaths;
        } else {
            cases_by_country_map.insert(id_key, case_by_country);
        }
    }
    cases_by_country_map
}

pub fn process_csv(
    confirmed: String,
    deaths: String,
    region: Region,
    today: String,
    global_current_cases: Option<(i64, i64)>,
) -> Result<
    (
        HashMap<String, CsvCase>,
        BTreeMap<usize, TimeSeriesCase>,
        (Option<String>, Option<String>, HighestCase, HighestCase),
    ),
    Box<dyn Error>,
> {
    let mut cases: HashMap<String, CsvCase> = HashMap::new();
    let mut countries_encountered: HashSet<String> = HashSet::new();
    let mut time_series_cases_map: BTreeMap<usize, TimeSeriesCase> = BTreeMap::new();
    let mut confirmed_csv_reader = csv::Reader::from_reader(confirmed.as_bytes());
    let mut deaths_csv_reader = csv::Reader::from_reader(deaths.as_bytes());

    let mut country_csv_header_index = 1;
    let mut province_csv_header_index = 0;
    let mut latitude_csv_header_index = 2;
    let mut longitude_csv_header_index = 3;
    let mut first_day_csv_header_index = 4;

    // US CSV has extra columns compared to global CSV
    if region == Region::US {
        country_csv_header_index = 7;
        province_csv_header_index = 6;
        latitude_csv_header_index = 8;
        longitude_csv_header_index = 9;
        first_day_csv_header_index = 11;
    }

    let mut global_date_first_case: Option<String> = None;
    let mut global_date_first_death: Option<String> = None;
    let mut highest_global_daily_confirmed = HighestCase {
        count: 0,
        date: None,
    };
    let mut highest_global_daily_deaths = HighestCase {
        count: 0,
        date: None,
    };

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
        let mut confirmed_today = 0;
        let mut deaths_today = 0;
        let mut date_first_case: Option<String> = None;
        let mut date_first_death: Option<String> = None;
        let mut highest_daily_confirmed = HighestCase {
            count: 0,
            date: None,
        };
        let mut highest_daily_deaths = HighestCase {
            count: 0,
            date: None,
        };

        for i in first_day_csv_header_index..confirmed_record.len() {
            let confirmed_cases = confirmed_record[i].parse::<i64>().unwrap_or_default();
            let confirmed_cases_yesterday =
                confirmed_record[i - 1].parse::<i64>().unwrap_or_default();
            // US death cases CSV has an extra column before dates and so needs to be shifted
            let deaths_index = match region {
                Region::US => i + 1,
                _ => i,
            };
            let death_cases = deaths_record[deaths_index]
                .parse::<i64>()
                .unwrap_or_default();
            let death_cases_yesterday = deaths_record[deaths_index - 1]
                .parse::<i64>()
                .unwrap_or_default();

            let day = &csv_headers[i];

            if i != first_day_csv_header_index {
                confirmed_today =
                    force_to_zero_if_negative(confirmed_cases - confirmed_cases_yesterday);
                deaths_today = force_to_zero_if_negative(death_cases - death_cases_yesterday);
            } else if i == first_day_csv_header_index {
                // First day of cases
                confirmed_today = confirmed_cases;
                deaths_today = death_cases;
                highest_daily_confirmed.count = confirmed_cases;
                highest_daily_deaths.count = death_cases;
            }

            if date_first_case.is_none() && confirmed_today > 0 {
                date_first_case = Some(day.to_string());
            }

            if date_first_death.is_none() && deaths_today > 0 {
                date_first_death = Some(day.to_string());
            }

            if confirmed_today > highest_daily_confirmed.count {
                highest_daily_confirmed.count = confirmed_today;
                highest_daily_confirmed.date = Some(day.to_string());
            }

            if deaths_today > highest_daily_deaths.count {
                highest_daily_deaths.count = deaths_today;
                highest_daily_deaths.date = Some(day.to_string());
            }

            let time_series_case = TimeSeriesCase::new(
                confirmed_cases,
                death_cases,
                confirmed_today,
                deaths_today,
                day.to_string(),
            );

            // Global CSV contains US as a row so no need to accumulate a US time series
            if region == Region::Global {
                if let Some(found_ts_case) = time_series_cases_map.get_mut(&i) {
                    found_ts_case.confirmed += confirmed_cases;
                    found_ts_case.deaths += death_cases;
                    found_ts_case.confirmedCasesToday += confirmed_today;
                    found_ts_case.deathsToday += deaths_today;
                    if found_ts_case.confirmedCasesToday > highest_global_daily_confirmed.count {
                        highest_global_daily_confirmed.count = found_ts_case.confirmedCasesToday;
                        highest_global_daily_confirmed.date = Some(found_ts_case.day.to_string());
                    }

                    if found_ts_case.deathsToday > highest_global_daily_deaths.count {
                        highest_global_daily_deaths.count = found_ts_case.deathsToday;
                        highest_global_daily_deaths.date = Some(found_ts_case.day.to_string());
                    }
                } else {
                    time_series_cases_map.insert(i, time_series_case.clone());
                }
            }

            time_series.push(time_series_case);
        }

        let province = match confirmed_record[province_csv_header_index].is_empty() {
            true => {
                if countries_encountered.contains(&confirmed_record[country_csv_header_index]) {
                    Some("mainland".to_string())
                } else {
                    None
                }
            }
            false => Some(confirmed_record[province_csv_header_index].to_string()),
        };
        let country = confirmed_record[country_csv_header_index].to_string();
        let country = patch_country_names(country);
        countries_encountered.insert(country.clone());
        let id_key = generate_id_key(&province, &country);

        if let Some(found_case) = cases.get_mut(&id_key) {
            found_case.cases = combine_time_series_cases(found_case.cases.clone(), time_series);
        } else {
            cases.insert(
                id_key,
                CsvCase {
                    Province_State: province,
                    Country_Region: country,
                    Lat: confirmed_record[latitude_csv_header_index]
                        .parse()
                        .unwrap_or_default(),
                    Long_: confirmed_record[longitude_csv_header_index]
                        .parse()
                        .unwrap_or_default(),
                    cases: time_series,
                    dateOfFirstCase: date_first_case,
                    dateOfFirstDeath: date_first_death,
                    highestDailyConfirmed: highest_daily_confirmed,
                    highestDailyDeaths: highest_daily_deaths,
                },
            );
        }
    }

    // Add the most recent days to global time series
    if let Some(global_current_cases) = global_current_cases {
        let (global_confirmed, global_deaths) = global_current_cases;
        let last_day_index = time_series_cases_map.len() + first_day_csv_header_index - 1;
        let global_confirmed_today = global_confirmed
            - time_series_cases_map
                .get(&last_day_index)
                .unwrap()
                .confirmed;
        let global_deaths_today =
            global_deaths - time_series_cases_map.get(&last_day_index).unwrap().deaths;
        let current_time_series_case = TimeSeriesCase::new(
            global_confirmed,
            global_deaths,
            force_to_zero_if_negative(global_confirmed_today),
            force_to_zero_if_negative(global_deaths_today),
            today.to_string(),
        );
        time_series_cases_map.insert(last_day_index + 1, current_time_series_case);
    }
    if let Some(first_confirmed) = time_series_cases_map
        .iter()
        .find(|(_, x)| x.confirmedCasesToday > 0)
    {
        global_date_first_case = Some(first_confirmed.1.day.to_string());
    }
    if let Some(first_death) = time_series_cases_map
        .iter()
        .find(|(_, x)| x.deathsToday > 0)
    {
        global_date_first_death = Some(first_death.1.day.to_string())
    }

    Ok((
        cases,
        time_series_cases_map,
        (
            global_date_first_case,
            global_date_first_death,
            highest_global_daily_confirmed,
            highest_global_daily_deaths,
        ),
    ))
}

pub fn process_global_cases_by_date(
    cases: &HashMap<String, CaseByLocation>,
    time_series_cases_map: &BTreeMap<usize, TimeSeriesCase>,
) -> BTreeMap<usize, GlobalDayCase> {
    let mut global_cases_by_date: BTreeMap<usize, GlobalDayCase> = BTreeMap::new();
    let mut i = 0;
    for (_ts_key, ts_case) in time_series_cases_map {
        let day = &ts_case.day;
        for (_case_key, country_case) in cases {
            if country_case.province.is_none()
                || country_case.province == Some("mainland".to_string())
                || country_case.province == Some("Greenland".to_string())
            {
                let global_case_by_date = GlobalCaseByDate {
                    idKey: country_case.idKey.clone(),
                    country: country_case.country.clone(),
                    countryCode: country_case.countryCode.clone(),
                    confirmed: country_case.casesByDate[i].confirmed,
                    deaths: country_case.casesByDate[i].deaths,
                    active: country_case.active,
                    recovered: country_case.recovered,
                    confirmedCasesToday: country_case.casesByDate[i].confirmedCasesToday,
                    deathsToday: country_case.casesByDate[i].deathsToday,
                };

                if let Some(global_day_case) = global_cases_by_date.get_mut(&i) {
                    global_day_case.casesOfTheDay.push(global_case_by_date);
                } else {
                    global_cases_by_date.insert(
                        i,
                        GlobalDayCase {
                            day: day.to_string(),
                            casesOfTheDay: Vec::from([global_case_by_date]),
                        },
                    );
                }
            }
        }
        i += 1;
    }
    global_cases_by_date
}
