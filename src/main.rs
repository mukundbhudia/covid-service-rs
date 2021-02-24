use ::std::io::Write;
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use mongodb::{bson, Client};
use std::error::Error;

pub mod data_processing;
pub mod data_sources;
pub mod logging;
pub mod schema;

use data_processing::{
    merge_csv_gis_cases, process_cases_by_country, process_csv, process_global_cases_by_date,
    process_owid_csv,
};
use data_sources::get_data_from_sources;
use logging::initialise_logging;
use schema::{Case, CaseByLocation, GlobalCaseByLocation, GlobalDayCase, Region, TimeSeriesCase};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let now: DateTime<Utc> = Utc::now();
    let today_m_d_y = now.format("%-m/%-d/%-y");
    let execution_time_start = Utc::now().time();

    initialise_logging();

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        writeln!(std::io::stderr(), "Incorrect number of arguments supplied.").unwrap();
        writeln!(
            std::io::stderr(),
            "Usage: covid-service-rs \"DB_URI\" \"DB_NAME\""
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} \"mongodb://localhost:27017/\" \"covid19\". For connection to local mongoDb instance to the database: covid19.",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    info!("Starting {}", args[0]);

    let db_uri = &args[1];
    let db_name = &args[2];

    let client = Client::with_uri_str(db_uri).await?;
    let db = client.database(db_name);
    let cases_collection = db.collection("casesByLocation");
    let totals_collection = db.collection("totals");
    let max_db_execution_seconds = 60;

    match db.list_collection_names(None).await {
        Ok(collections) => {
            debug!("{} collection(s) found in the database.", collections.len());
        }
        Err(error) => {
            error!("Error connecting to the database with details: {:?}", error);
            std::process::exit(1);
        }
    }

    let (
        cases_by_country,
        confirmed_global_cases,
        deaths_global_cases,
        confirmed_us_cases,
        deaths_us_cases,
        owid_data,
        global_confirmed,
        global_recovered,
        global_deaths,
    ) = get_data_from_sources().await?;
    let (owid_data, global_owid_stats) = process_owid_csv(owid_data)?;

    let global_population = global_owid_stats.population;
    let net_req_time_stop = Utc::now().time();
    let core_processing_time_start = Utc::now().time();

    info!(
        "Total confirmed: {:?}, recovered: {:?}, deaths: {:?}",
        global_confirmed, global_recovered, global_deaths,
    );

    let (mut processed_csv, global_time_series_map, global_csv_stats) = process_csv(
        confirmed_global_cases,
        deaths_global_cases,
        Region::Global,
        today_m_d_y.to_string(),
        Some((global_confirmed, global_deaths, global_population)),
    )?;
    let (
        date_of_first_confirmed,
        date_of_first_death,
        highest_daily_confirmed,
        highest_daily_deaths,
        global_confirmed_per_capita,
        global_deaths_per_capita,
    ) = global_csv_stats;
    let (us_processed_csv, _, _) = process_csv(
        confirmed_us_cases,
        deaths_us_cases,
        Region::US,
        today_m_d_y.to_string(),
        None,
    )?;

    info!("GIS cases: {}", cases_by_country.len());

    info!("Days since first cases: {}", global_time_series_map.len());

    info!(
        "{} Global and {} US CSV cases. Global population: {}.",
        processed_csv.len(),
        us_processed_csv.len(),
        global_population,
    );

    processed_csv.extend(us_processed_csv);

    let cases_by_country = cases_by_country
        .iter()
        .cloned()
        .map(|x| x.attributes)
        .collect::<Vec<Case>>();
    let cases_by_country = process_cases_by_country(cases_by_country);
    let cases_by_location_map = merge_csv_gis_cases(
        processed_csv,
        cases_by_country,
        owid_data,
        today_m_d_y.to_string(),
    );

    let global_day_cases =
        process_global_cases_by_date(&cases_by_location_map, &global_time_series_map);
    let global_day_cases = global_day_cases
        .values()
        .cloned()
        .collect::<Vec<GlobalDayCase>>();

    let cases_by_location = cases_by_location_map
        .values()
        .cloned()
        .collect::<Vec<CaseByLocation>>();

    let global_time_series = global_time_series_map
        .values()
        .cloned()
        .collect::<Vec<TimeSeriesCase>>();

    let global_confirmed_yesterday = global_time_series
        .get(global_time_series.len() - 2)
        .unwrap()
        .confirmed;
    let global_deaths_yesterday = global_time_series
        .get(global_time_series.len() - 2)
        .unwrap()
        .deaths;

    let global_confirmed_today = global_confirmed - global_confirmed_yesterday;
    let global_deaths_today = global_deaths - global_deaths_yesterday;

    info!(
        "Confirmed today: {}, deaths today: {}",
        global_confirmed_today, global_deaths_today
    );

    let global_cases = GlobalCaseByLocation {
        active: global_confirmed - (global_recovered + global_deaths),
        confirmed: global_confirmed,
        recovered: global_recovered,
        deaths: global_deaths,
        confirmedCasesToday: global_confirmed_today,
        deathsToday: global_deaths_today,
        confirmedPerCapita: global_confirmed_per_capita,
        deathsPerCapita: global_deaths_per_capita,
        globalPopulation: global_population,
        populationDensity: global_owid_stats.population_density,
        medianAge: global_owid_stats.median_age,
        aged65older: global_owid_stats.aged_65_older,
        aged70older: global_owid_stats.aged_70_older,
        gdpPerCapita: global_owid_stats.gdp_per_capita,
        diabetesPrevalence: global_owid_stats.diabetes_prevalence,
        cardiovascDeathRate: global_owid_stats.cardiovasc_death_rate,
        lifeExpectancy: Some(global_owid_stats.life_expectancy),
        humanDevelopmentIndex: global_owid_stats.human_development_index,
        totalTests: global_owid_stats.total_tests,
        totalTestsPerThousand: global_owid_stats.total_tests_per_thousand,
        totalVaccinations: global_owid_stats.total_vaccinations,
        peopleVaccinated: global_owid_stats.people_vaccinated,
        peopleFullyVaccinated: global_owid_stats.people_fully_vaccinated,
        totalVaccinationsPerHundred: global_owid_stats.total_vaccinations_per_hundred,
        peopleVaccinatedPerHundred: global_owid_stats.people_vaccinated_per_hundred,
        peopleFullyVaccinatedPerHundred: global_owid_stats.people_fully_vaccinated_per_hundred,
        reproductionRate: global_owid_stats.reproduction_rate,
        positiveRate: global_owid_stats.positive_rate,
        femaleSmokers: global_owid_stats.female_smokers,
        maleSmokers: global_owid_stats.male_smokers,
        handwashingFacilities: global_owid_stats.handwashing_facilities,
        hospitalBedsPerThousand: global_owid_stats.hospital_beds_per_thousand,
        extremePoverty: global_owid_stats.extreme_poverty,
        timeSeriesTotalCasesByDate: global_time_series,
        globalCasesByDate: global_day_cases,
        dateOfFirstCase: date_of_first_confirmed,
        dateOfFirstDeath: date_of_first_death,
        highestDailyConfirmed: highest_daily_confirmed,
        highestDailyDeaths: highest_daily_deaths,
        timeStamp: From::from(Utc::now()),
    };

    let core_processing_time_stop = Utc::now().time();
    let db_time_start = Utc::now().time();

    debug!("Saving to db...");

    let save_to_db_totals_task = tokio::spawn(async move {
        let global_cases_bson = bson::to_document(&global_cases).unwrap();
        totals_collection.drop(None).await.unwrap();
        totals_collection
            .insert_one(global_cases_bson, None)
            .await
            .unwrap();
    });

    let save_to_db_cases_task = tokio::spawn(async move {
        let processed_cases_by_location = cases_by_location
            .iter()
            .map(|x| bson::to_document(&x).unwrap())
            .collect::<Vec<_>>();
        cases_collection.drop(None).await.unwrap();
        cases_collection
            .insert_many(processed_cases_by_location, None)
            .await
            .unwrap();
    });

    tokio::try_join!(save_to_db_totals_task, save_to_db_cases_task)?;

    let db_time_stop = Utc::now().time();
    debug!("Saved to DB");

    let execution_time_stop = Utc::now().time();
    let elapsed_execution_time = execution_time_stop - execution_time_start;
    let elapsed_core_time = core_processing_time_stop - core_processing_time_start;
    let elapsed_net_req_time = net_req_time_stop - execution_time_start;
    let elapsed_db_time = db_time_stop - db_time_start;

    if elapsed_db_time.num_seconds() > max_db_execution_seconds {
        warn!(
            "DB processing and execution took longer than {} seconds.",
            max_db_execution_seconds
        );
    }

    info!(
        "Script took {} seconds ({} milliseconds). {} second(s) ({} milliseconds) of network requests. {} second(s) ({} milliseconds) of db processing. {} second(s) ({} milliseconds) of core processing.",
        elapsed_execution_time.num_seconds(),
        elapsed_execution_time.num_milliseconds(),
        elapsed_net_req_time.num_seconds(),
        elapsed_net_req_time.num_milliseconds(),
        elapsed_db_time.num_seconds(),
        elapsed_db_time.num_milliseconds(),
        elapsed_core_time.num_seconds(),
        elapsed_core_time.num_milliseconds(),
    );

    Ok(())
}
