use serde::Deserialize;
use std::error::Error;
// use log;
// use simple_logger::SimpleLogger;

// #[derive(Deserialize, Debug)]
// #[allow(non_snake_case)]
// struct User {
//     userId: u32,
//     id: u32,
//     title: String
// }

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Case {
    Province_State : Option<String>,
    Country_Region : String,
    Last_Update : Option<i64>,
    Lat : Option<f64>,
    Long_ : Option<f64>,
    Confirmed : i64,
    Recovered : i64,
    Deaths : i64,
    Active : i64,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Attributes {
    attributes: Case,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct CasesByCountry {
    features: Vec<Attributes>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // SimpleLogger::new().init().unwrap();

let gis_service = String::from("https://services1.arcgis.com/0MSEUqKaxRlEPj5g/arcgis/rest/services/ncov_cases/FeatureServer/1/query");
let cases_by_country_query_params = String::from("?where=1%3D1&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=true&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
// let total_confirmed_cases_query_params = String::from("?where=(Confirmed+>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Confirmed'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
// let total_recovered_cases_query_params = String::from("?where=(Confirmed+>+0)+AND+(Recovered+<>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Recovered'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
// let total_death_cases_query_params = String::from("?where=(Confirmed+>+0)+AND+(Deaths+>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Deaths'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");


    let cases_by_country_url = format!("{}{}", gis_service, cases_by_country_query_params);
    let cases_by_country_response = reqwest::get(&cases_by_country_url).await?;

    let cases_by_country: CasesByCountry = cases_by_country_response.json().await?;
    println!("cases_by_country {:?}", cases_by_country.features);

    // let users: Vec<User> = response.json().await?;
    // println!("Found {:?} users from {}", users.len(), cases_by_country_url);
    // log::info!("found {} users", users.len());


    let csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_global.csv");
    let csv_response = reqwest::get(&csv_request_url).await?;

    let confirmed_global_cases = csv_response.text().await?;

    let mut reader = csv::Reader::from_reader(confirmed_global_cases.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "{} has lat: {} and long: {} with {} days of data",
            // &record[0], // Province data
            &record[1],
            &record[2],
            &record[3],
            record.len() - 4,
        );
    }

    Ok(())
}
