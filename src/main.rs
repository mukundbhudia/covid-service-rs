use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    userId: u32,
    id: u32,
    title: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = String::from("https://jsonplaceholder.typicode.com/posts");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
