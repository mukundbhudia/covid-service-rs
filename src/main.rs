use serde::Deserialize;
use std::error::Error;
// use reqwest::Error;
use log;
use simple_logger::SimpleLogger;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct User {
    userId: u32,
    id: u32,
    title: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    let request_url = String::from("https://jsonplaceholder.typicode.com/posts");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    log::info!("found {} users", users.len());


    let csv_request_url = String::from("https://raw.githubusercontent.com/CSSEGISandData/COVID-19/master/csse_covid_19_data/csse_covid_19_time_series/time_series_covid19_confirmed_global.csv");
    println!("{}", csv_request_url);
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
