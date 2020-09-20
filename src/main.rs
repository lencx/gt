// use std::collections::HashMap;
// use reqwest::header::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = reqwest::get("https://httpbin.org/ip")
    let params = [
        ("q", "created:2020-09-18T00:00:00%2B08:00..2020-09-20T00:00:00%2B08:00"),
        ("sort", "stars"),
        ("order", "desc"),
    ];
    // let mut headers = HeaderMap::new();
    // headers.insert(CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
    let client = reqwest::Client::new();
    let resp = client.get("https://api.github.com/search/repositories")
        .query(&params)
        .send()
        .await?;
        // .json::<HashMap<String, String>>()
        // .await?;
    println!("{:#?}", resp);
    Ok(())
}
