use crate::schema::{CasesByCountry, Total};
use std::error::Error;

fn get_sources() -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
) {
    let gis_service = String::from("https://services1.arcgis.com/0MSEUqKaxRlEPj5g/arcgis/rest/services/ncov_cases/FeatureServer/1/query");
    let cases_by_country_query_params = String::from("?where=1%3D1&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=true&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
    let total_confirmed_cases_query_params = String::from("?where=(Confirmed+>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Confirmed'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
    let total_recovered_cases_query_params = String::from("?where=(Confirmed+>+0)+AND+(Recovered+<>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Recovered'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");
    let total_death_cases_query_params = String::from("?where=(Confirmed+>+0)+AND+(Deaths+>+0)&objectIds=&time=&geometry=&geometryType=esriGeometryEnvelope&inSR=&spatialRel=esriSpatialRelIntersects&resultType=none&distance=0.0&units=esriSRUnit_Meter&returnGeodetic=false&outFields=*&returnGeometry=false&featureEncoding=esriDefault&multipatchOption=xyFootprint&maxAllowableOffset=&geometryPrecision=&outSR=&datumTransformation=&applyVCSProjection=false&returnIdsOnly=false&returnUniqueIdsOnly=false&returnCountOnly=false&returnExtentOnly=false&returnQueryGeometry=false&returnDistinctValues=false&cacheHint=false&orderByFields=&groupByFieldsForStatistics=&outStatistics=[{'statisticType'%3A'sum'%2C'onStatisticField'%3A'Deaths'%2C'outStatisticFieldName'%3A'value'}]&having=&resultOffset=&resultRecordCount=&returnZ=false&returnM=false&returnExceededLimitFeatures=true&quantizationParameters=&sqlFormat=none&f=pjson&token=");

    let confirmed_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_global.csv");
    let deaths_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_deaths_global.csv");

    let confirmed_us_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_US.csv");
    let deaths_us_csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_deaths_US.csv");

    let cases_by_country_url = format!("{}{}", gis_service, cases_by_country_query_params);
    let total_confirmed_url = format!("{}{}", gis_service, total_confirmed_cases_query_params);
    let total_recovered_url = format!("{}{}", gis_service, total_recovered_cases_query_params);
    let total_deaths_url = format!("{}{}", gis_service, total_death_cases_query_params);

    (
        cases_by_country_url,
        confirmed_csv_request_url,
        deaths_csv_request_url,
        confirmed_us_csv_request_url,
        deaths_us_csv_request_url,
        total_confirmed_url,
        total_recovered_url,
        total_deaths_url,
    )
}

pub async fn get_data_from_sources() -> Result<
    (
        CasesByCountry,
        String,
        String,
        String,
        String,
        i64,
        i64,
        i64,
    ),
    Box<dyn Error>,
> {
    let (
        cases_by_country_url,
        confirmed_csv_request_url,
        deaths_csv_request_url,
        confirmed_us_csv_request_url,
        deaths_us_csv_request_url,
        total_confirmed_url,
        total_recovered_url,
        total_deaths_url,
    ) = get_sources();

    let cases_by_country_response = reqwest::get(&cases_by_country_url).await?;
    let cases_by_country: CasesByCountry = cases_by_country_response.json().await?;

    let confirmed_csv_response = reqwest::get(&confirmed_csv_request_url).await?;
    let confirmed_global_cases = confirmed_csv_response.text().await?;

    let deaths_csv_response = reqwest::get(&deaths_csv_request_url).await?;
    let deaths_global_cases = deaths_csv_response.text().await?;

    let confirmed_us_csv_response = reqwest::get(&confirmed_us_csv_request_url).await?;
    let confirmed_us_cases = confirmed_us_csv_response.text().await?;

    let deaths_us_csv_response = reqwest::get(&deaths_us_csv_request_url).await?;
    let deaths_us_cases = deaths_us_csv_response.text().await?;

    let total_confirmed_response = reqwest::get(&total_confirmed_url).await?;
    let total_confirmed: Total = total_confirmed_response.json().await?;

    let total_recovered_response = reqwest::get(&total_recovered_url).await?;
    let total_recovered: Total = total_recovered_response.json().await?;

    let total_deaths_response = reqwest::get(&total_deaths_url).await?;
    let total_deaths: Total = total_deaths_response.json().await?;

    let global_confirmed = total_confirmed.features[0].attributes.value;
    let global_recovered = total_recovered.features[0].attributes.value;
    let global_deaths = total_deaths.features[0].attributes.value;

    Ok((
        cases_by_country,
        confirmed_global_cases,
        deaths_global_cases,
        confirmed_us_cases,
        deaths_us_cases,
        global_confirmed,
        global_recovered,
        global_deaths,
    ))
}
