use serde::Deserialize;
use reqwest::Error;
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
async fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();

    let request_url = String::from("https://jsonplaceholder.typicode.com/posts");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    log::info!("found {} users", users.len());
    Ok(())
}
