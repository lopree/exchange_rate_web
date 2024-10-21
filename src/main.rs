use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize,Serialize)]
struct ExchangeRate {
    rates: HashMap<String, f64>,
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let routes = warp::path!("api" / "rates")
        .and(warp::any().map(move || client.clone()))
        .and_then(fetch_and_return_rates);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

async fn fetch_and_return_rates(client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match fetch_rates(&client).await {
        Ok(rates) => Ok(warp::reply::json(&rates)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn fetch_rates(client: &Client) -> Result<HashMap<String, f64>, reqwest::Error> {
    let response = client.get("https://api.exchangerate-api.com/v4/latest/CNY")
        .send()
        .await?
        .json::<ExchangeRate>()
        .await?;
    Ok(response.rates)
}
