use chrono::Utc;
use mongodb::{bson, Client};
use std::error::Error;

pub mod alpha3_country_codes;
use alpha3_country_codes::alpha_codes;

pub mod schema;
use schema::{CasesByCountry, CsvCase, GlobalCaseByLocation, Total};

pub mod data_processing;
use data_processing::process_csv;

// use log;
// use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let execution_start = Utc::now().time();
    // SimpleLogger::new().init().unwrap();

    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let db = client.database("covid19r");
    let cases_collection = db.collection("casesByLocation");
    let totals_collection = db.collection("totals");

    // for coll_name in db.list_collection_names(None).await? {
    //     println!("collection: {}", coll_name);
    // }

    let country_alpha_codes = alpha_codes();
    println!(
        "{:?} alpha3 county codes",
        country_alpha_codes.values().count()
    );

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

    let confirmed_us_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_global.csv");
    let confirmed_us_csv_response = reqwest::get(&confirmed_us_csv_request_url).await?;
    let _confirmed_us_cases = confirmed_us_csv_response.text().await?;

    let deaths_us_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_deaths_global.csv");
    let deaths_us_csv_response = reqwest::get(&deaths_us_csv_request_url).await?;
    let _deaths_us_cases = deaths_us_csv_response.text().await?;

    let processed_csv: Vec<CsvCase> = process_csv(confirmed_global_cases, deaths_global_cases)?;
    println!("{:?} CSV cases... ", processed_csv.len());

    let total_confirmed_url = format!("{}{}", gis_service, total_confirmed_cases_query_params);
    let total_confirmed_response = reqwest::get(&total_confirmed_url).await?;
    let total_confirmed: Total = total_confirmed_response.json().await?;

    let total_recovered_url = format!("{}{}", gis_service, total_recovered_cases_query_params);
    let total_recovered_response = reqwest::get(&total_recovered_url).await?;
    let total_recovered: Total = total_recovered_response.json().await?;

    let total_deaths_url = format!("{}{}", gis_service, total_death_cases_query_params);
    let total_deaths_response = reqwest::get(&total_deaths_url).await?;
    let total_deaths: Total = total_deaths_response.json().await?;

    let global_confirmed = total_confirmed.features[0].attributes.value;
    let global_recovered = total_recovered.features[0].attributes.value;
    let global_deaths = total_deaths.features[0].attributes.value;

    println!(
        "Total confirmed: {:?}, recovered: {:?}, deaths: {:?}",
        global_confirmed, global_recovered, global_deaths,
    );

    let global_cases = GlobalCaseByLocation {
        active: global_confirmed - (global_recovered + global_deaths),
        confirmed: global_confirmed,
        recovered: global_recovered,
        deaths: global_deaths,
        confirmedCasesToday: 0,
        deathsToday: 0,
        lastUpdate: "".to_string(),
        timeSeriesTotalCasesByDate: Vec::new(),
        timeStamp: From::from(Utc::now()),
    };

    let global_cases_bson = bson::to_document(&global_cases).unwrap();

    let processed_csv_bson = processed_csv
        .iter()
        .map(|x| bson::to_document(&x).unwrap())
        .collect::<Vec<_>>();
    cases_collection.drop(None).await?;
    totals_collection.drop(None).await?;
    cases_collection
        .insert_many(processed_csv_bson, None)
        .await?;

    totals_collection
        .insert_one(global_cases_bson, None)
        .await?;

    println!("Saved to DB");

    let execution_stop = Utc::now().time();
    let elapsed_execution_time = execution_stop - execution_start;
    println!(
        "Script took {} seconds and {} milliseconds",
        elapsed_execution_time.num_seconds(),
        elapsed_execution_time.num_milliseconds(),
    );

    Ok(())
}
