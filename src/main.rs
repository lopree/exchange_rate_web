use warp::Filter;
use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// 定义汇率结构体
#[derive(Deserialize, Serialize)]
struct ExchangeRate {
    rates: HashMap<String, f64>,
}

// 定义我们关心的货币
const SELECTED_CURRENCIES: &[&str] = &[
    "USD", "GBP", "JPY", "ARS", "INR", "TRY", "HKD", "TWD", "SGD"
];

// 获取汇率数据并返回自定义的货币汇率
async fn fetch_and_return_rates(client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match fetch_rates(&client).await {
        Ok(rates) => {
            let filtered_rates: HashMap<String, f64> = rates
                .into_iter()
                .filter(|(currency, _)| SELECTED_CURRENCIES.contains(&currency.as_str()))
                .collect();

            Ok(warp::reply::json(&filtered_rates))
        }
        Err(_) => Err(warp::reject::not_found()),
    }
}

// 调用外部 API 获取实时汇率
async fn fetch_rates(client: &Client) -> Result<HashMap<String, f64>, reqwest::Error> {
    let response = client
        .get("https://api.exchangerate-api.com/v4/latest/CNY") // 基于人民币的汇率
        .send()
        .await?
        .json::<ExchangeRate>()
        .await?;

    Ok(response.rates)
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    // 创建 API 路由，返回汇率信息
    let rates_route = warp::path!("api" / "rates")
        .and(warp::any().map(move || client.clone()))
        .and_then(fetch_and_return_rates);

    // 静态文件路由
    let static_files = warp::fs::dir("src");

    // 主页路由
    let hello = warp::path::end()
        .map(|| warp::reply::html(include_str!("index.html")));

    // 使用 warp::any 来组合路由
    let routes = warp::any()
        .and(hello.or(rates_route).or(static_files));

    // 运行服务
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
