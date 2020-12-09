use ::std::io::Write;
use chrono::Utc;
use mongodb::{bson, Client};
use std::error::Error;

pub mod alpha3_country_codes;
pub mod data_processing;
pub mod schema;

use data_processing::{merge_csv_gis_cases, process_cases_by_country, process_csv};
use schema::{Case, CasesByCountry, GlobalCaseByLocation, Region, TimeSeriesCase, Total};

// use log;
// use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let execution_time_start = Utc::now().time();
    // SimpleLogger::new().init().unwrap();

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

    let db_uri = &args[1];
    let db_name = &args[2];

    let client = Client::with_uri_str(db_uri).await?;
    let db = client.database(db_name);
    let cases_collection = db.collection("casesByLocation");
    let totals_collection = db.collection("totals");

    // for coll_name in db.list_collection_names(None).await? {
    //     println!("collection: {}", coll_name);
    // }

    let gis_service = String::from("https://services1.arcgis.com/0MSEUqKaxRlEPj5g/arcgis/rest/services/ncov_cases/FeatureServer/1/query");
    let cases_by_country_query_params = String::from("?where=1%3D1&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=true&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
    let total_confirmed_cases_query_params = String::from("?where=(Confirmed+>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Confirmed'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
    let total_recovered_cases_query_params = String::from("?where=(Confirmed+>+0)+AND+(Recovered+<>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Recovered'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
    let total_death_cases_query_params = String::from("?where=(Confirmed+>+0)+AND+(Deaths+>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Deaths'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");

    let cases_by_country_url = format!("{}{}", gis_service, cases_by_country_query_params);
    let cases_by_country_response = reqwest::get(&cases_by_country_url).await?;
    let cases_by_country: CasesByCountry = cases_by_country_response.json().await?;
    println!("cases_by_country {:?}", cases_by_country.features.len());

    let confirmed_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_global.csv");
    let confirmed_csv_response = reqwest::get(&confirmed_csv_request_url).await?;
    let confirmed_global_cases = confirmed_csv_response.text().await?;

    let deaths_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_deaths_global.csv");
    let deaths_csv_response = reqwest::get(&deaths_csv_request_url).await?;
    let deaths_global_cases = deaths_csv_response.text().await?;

    let confirmed_us_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_US.csv");
    let confirmed_us_csv_response = reqwest::get(&confirmed_us_csv_request_url).await?;
    let confirmed_us_cases = confirmed_us_csv_response.text().await?;

    let deaths_us_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_deaths_US.csv");
    let deaths_us_csv_response = reqwest::get(&deaths_us_csv_request_url).await?;
    let deaths_us_cases = deaths_us_csv_response.text().await?;

    let total_confirmed_url = format!("{}{}", gis_service, total_confirmed_cases_query_params);
    let total_confirmed_response = reqwest::get(&total_confirmed_url).await?;
    let total_confirmed: Total = total_confirmed_response.json().await?;

    let total_recovered_url = format!("{}{}", gis_service, total_recovered_cases_query_params);
    let total_recovered_response = reqwest::get(&total_recovered_url).await?;
    let total_recovered: Total = total_recovered_response.json().await?;

    let total_deaths_url = format!("{}{}", gis_service, total_death_cases_query_params);
    let total_deaths_response = reqwest::get(&total_deaths_url).await?;
    let total_deaths: Total = total_deaths_response.json().await?;

    let net_req_time_stop = Utc::now().time();
    let core_processing_time_start = Utc::now().time();

    let (mut processed_csv, global_time_series_map) =
        process_csv(confirmed_global_cases, deaths_global_cases, Region::Global)?;

    let (us_processed_csv, us_time_series_map) =
        process_csv(confirmed_us_cases, deaths_us_cases, Region::US)?;

    println!(
        "{:?} Global and {:?} US CSV cases... ",
        processed_csv.len(),
        us_processed_csv.len()
    );

    processed_csv.extend(us_processed_csv);

    let cases_by_country = cases_by_country
        .features
        .iter()
        .cloned()
        .map(|x| x.attributes)
        .collect::<Vec<Case>>();
    let cases_by_country = process_cases_by_country(cases_by_country);
    let cases_by_location = merge_csv_gis_cases(processed_csv, cases_by_country);

    let global_confirmed = total_confirmed.features[0].attributes.value;
    let global_recovered = total_recovered.features[0].attributes.value;
    let global_deaths = total_deaths.features[0].attributes.value;

    println!(
        "Total confirmed: {:?}, recovered: {:?}, deaths: {:?}",
        global_confirmed, global_recovered, global_deaths,
    );

    let global_time_series = global_time_series_map
        .values()
        .cloned()
        .collect::<Vec<TimeSeriesCase>>();

    let global_confirmed_yesterday = global_time_series.last().unwrap().confirmed;
    let global_deaths_yesterday = global_time_series.last().unwrap().deaths;
    let global_confirmed_today = global_confirmed - global_confirmed_yesterday;
    let global_deaths_today = global_deaths - global_deaths_yesterday;

    println!(
        "Confirmed yesterday: {}, deaths yesterday: {}. \nConfirmed today: {}, deaths today: {}",
        global_confirmed_yesterday,
        global_deaths_yesterday,
        global_confirmed_today,
        global_deaths_today
    );

    let global_cases = GlobalCaseByLocation {
        active: global_confirmed - (global_recovered + global_deaths),
        confirmed: global_confirmed,
        recovered: global_recovered,
        deaths: global_deaths,
        confirmedCasesToday: global_confirmed_today,
        deathsToday: global_deaths_today,
        timeSeriesTotalCasesByDate: global_time_series,
        globalCasesByDate: Vec::new(),
        timeStamp: From::from(Utc::now()),
    };

    let global_cases_bson = bson::to_document(&global_cases).unwrap();
    let processed_cases_by_location = cases_by_location
        .iter()
        .map(|x| bson::to_document(&x).unwrap())
        .collect::<Vec<_>>();

    let core_processing_time_stop = Utc::now().time();
    let db_time_start = Utc::now().time();

    cases_collection.drop(None).await?;
    totals_collection.drop(None).await?;
    cases_collection
        .insert_many(processed_cases_by_location, None)
        .await?;

    totals_collection
        .insert_one(global_cases_bson, None)
        .await?;

    let db_time_stop = Utc::now().time();
    println!("Saved to DB\n");

    let execution_time_stop = Utc::now().time();
    let elapsed_execution_time = execution_time_stop - execution_time_start;
    let elapsed_core_time = core_processing_time_stop - core_processing_time_start;
    let elapsed_net_req_time = net_req_time_stop - execution_time_start;
    let elapsed_db_time = db_time_stop - db_time_start;

    println!(
        "Script took {} seconds ({} milliseconds).\n {} second(s) ({} milliseconds) of network requests. \n {} second(s) ({} milliseconds) of db processing. \n {} second(s) ({} milliseconds) of core processing.",
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
