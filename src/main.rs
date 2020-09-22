use reqwest::header::*;

// https://github.com/settings/tokens/new?description=gt&scopes=public_repo

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = [
        ("q", "1"),
        ("sort", "stars"),
        ("order", "desc"),
    ];
    let mut headers = HeaderMap::new();
    // headers.insert("Authorization", "xxxxxx".parse().unwrap());
    headers.insert("Host", "api.github.com".parse().unwrap());
    // Error: status: 403, when curl returns 200
    // see: https://github.com/seanmonstar/reqwest/issues/918
    headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.102 Safari/537.36".parse().unwrap());
    let client = reqwest::Client::new();
    let resp = client.get("https://api.github.com/search/repositories")
        .query(&params)
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:?}", resp);
    Ok(())
}
