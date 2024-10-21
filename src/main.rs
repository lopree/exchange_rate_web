use warp::Filter;
use reqwest::Client;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExchangeRate {
    rates: HashMap<String, f64>,
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    // 创建 API 路由，返回汇率信息
    let rates_route = warp::path!("api" / "rates")
        .and(warp::any().map(move || client.clone()))
        .and_then(fetch_and_return_rates);

    // 创建主页路由，返回 index.html 文件
    let hello = warp::path::end()
        .map(|| warp::reply::html(include_str!("index.html")));

    // 将主页路由和 API 路由结合
    let routes = hello.or(rates_route);

    // 启动服务器，监听 0.0.0.0:3030
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

// 获取汇率数据并返回
async fn fetch_and_return_rates(client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match fetch_rates(&client).await {
        Ok(rates) => Ok(warp::reply::json(&rates)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

// 调用外部 API 获取实时汇率
async fn fetch_rates(client: &Client) -> Result<HashMap<String, f64>, reqwest::Error> {
    let response = client
        .get("https://api.exchangerate-api.com/v4/latest/CNY")
        .send()
        .await?
        .json::<ExchangeRate>()
        .await?;
    Ok(response.rates)
}
