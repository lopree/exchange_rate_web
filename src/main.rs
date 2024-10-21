use warp::Filter;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize,Serialize)]
struct RateResponse {
    rates: HashMap<String, f64>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() {
    let rates = warp::path("api")
        .and(warp::path("rates"))
        .map(|| {
            let rates = serde_json::json!({
                "CNY": 1.0, // 人民币的基准汇率
                "USD": 0.15 // 假设美元的汇率（1 CNY = 0.15 USD）
            });
            warp::reply::json(&rates)
        });

    let routes = rates;

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

// 错误处理
async fn handle_rejection(err: warp::Rejection) -> Result<warp::reply::Response, warp::Rejection> {
    // 这里可以添加更多错误处理逻辑
    eprintln!("Unhandled rejection: {:?}", err);
    Err(err)
}

async fn fetch_rates(client: &Client) -> Result<RateResponse, reqwest::Error> {
    let url = "https://api.exchangerate-api.com/v4/latest/USD"; // 示例 API
    let response = client.get(url).send().await?;
    let rates = response.json::<RateResponse>().await?;
    Ok(rates)
}
