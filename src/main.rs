// use std::collections::HashMap;
use reqwest::header::*;

// https://github.com/settings/tokens/new?description=gt&scopes=public_repo

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = reqwest::get("https://httpbin.org/ip")
    let params = [
        ("q", "created:2020-09-18T00:00:00%2B08:00..2020-09-20T00:00:00%2B08:00"),
        ("sort", "stars"),
        ("order", "desc"),
    ];
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "token xxxxxxx".parse().unwrap());
    let client = reqwest::Client::new();
    let resp = client.get("https://api.github.com/search/repositories")
        .query(&params)
        .headers(headers)
        .send()
        .await?;
        // .json::<HashMap<String, String>>()
        // .await?;
    println!("{:#?}", resp);
    Ok(())
}
