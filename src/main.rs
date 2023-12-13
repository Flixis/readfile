use reqwest;
use serde_json::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Platform {
    signature: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Release {
    version: String,
    notes: String,
    pub_date: String,
    platforms: HashMap<String, Platform>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://gist.githubusercontent.com/Flixis/4797c2696d2525950248bafc4b9a90f5/raw/";

    let res = reqwest::get(url).await.unwrap();
    let body = res.json::<Release>().await.unwrap();

    println!("{:#?}", &body);

    Ok(())
}
