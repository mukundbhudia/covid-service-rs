use mongodb::{bson, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;
use time::now;
// use log;
// use simple_logger::SimpleLogger;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct TimeSeriesCase {
    confirmed: i64,
    deaths: i64,
    confirmedToday: i64,
    deathsToday: i64,
    day: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Case {
    Province_State: Option<String>,
    Country_Region: String,
    Last_Update: Option<i64>,
    Lat: Option<f64>,
    Long_: Option<f64>,
    Confirmed: i64,
    Recovered: i64,
    Deaths: i64,
    Active: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct CsvCase {
    Province_State: Option<String>,
    Country_Region: String,
    Lat: f64,
    Long_: f64,
    cases: Vec<TimeSeriesCase>,
}

#[derive(Deserialize, Debug)]
struct TotalsValue {
    value: i64,
}

#[derive(Deserialize, Debug)]
struct TotalsAttributes {
    attributes: TotalsValue,
}

#[derive(Deserialize, Debug)]
struct Total {
    features: Vec<TotalsAttributes>,
}

#[derive(Deserialize, Debug)]
struct CasesAttributes {
    attributes: Case,
}

#[derive(Deserialize, Debug)]
struct CasesByCountry {
    features: Vec<CasesAttributes>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let execution_start = now();
    // SimpleLogger::new().init().unwrap();

    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let db = client.database("covid19r");
    let cases_collection = db.collection("casesByLocation");
    // let totals_collection = db.collection("totals");

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

    println!(
        "Total confirmed: {:?}, recovered: {:?}, deaths: {:?}",
        total_confirmed.features[0].attributes.value,
        total_recovered.features[0].attributes.value,
        total_deaths.features[0].attributes.value,
    );

    let processed_csv_bson = processed_csv
        .iter()
        .map(|x| bson::to_document(&x).unwrap())
        .collect::<Vec<_>>();
    cases_collection.insert_many(processed_csv_bson, None).await?;
    println!("Saved to DB");

    let execution_stop = now();
    println!(
        "Script took {} seconds",
        execution_stop.to_timespec().sec - execution_start.to_timespec().sec
    );

    Ok(())
}

fn process_csv(confirmed: String, deaths: String) -> Result<Vec<CsvCase>, Box<dyn Error>> {
    let mut cases = Vec::new();
    let mut confirmed_csv_reader = csv::Reader::from_reader(confirmed.as_bytes());
    let mut deaths_csv_reader = csv::Reader::from_reader(deaths.as_bytes());
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
        for i in 4..confirmed_record.len() {
            time_series.push(TimeSeriesCase {
                confirmed: confirmed_record[i].parse().unwrap_or_default(),
                deaths: deaths_record[i].parse().unwrap_or_default(),
                confirmedToday: 0,
                deathsToday: 0,
                day: csv_headers[i].to_string(),
            });
        }
        cases.push(CsvCase {
            Province_State: match confirmed_record[0].is_empty() {
                true => None,
                false => Some(confirmed_record[0].to_string()),
            },
            Country_Region: confirmed_record[1].to_string(),
            Lat: confirmed_record[2].parse().unwrap_or_default(),
            Long_: confirmed_record[3].parse().unwrap_or_default(),
            cases: time_series,
        });
    }
    Ok(cases)
}
