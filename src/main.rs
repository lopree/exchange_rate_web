use reqwest::Client;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize,Serialize)]
struct ExchangeRate {
    rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    // 创建汇率查询路由
    let rates_route = warp::path("api")
        .and(warp::path("rates"))
        .and_then(move || {
            let client = client.clone();
            async move {
                match fetch_rates(&client).await {
                    Ok(rates) => Ok(warp::reply::json(&rates)),
                    Err(_) => Err(warp::reject::not_found()),
                }
            }
        });

    // 创建主页路由
    let hello = warp::path::end()
        .map(|| warp::reply::html(include_str!("index.html")));

    // 启动服务器
    let routes = hello.or(rates_route);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn fetch_rates(client: &Client) -> Result<ExchangeRate, reqwest::Error> {
    // 这里使用一个示例 API，替换为你选择的 API
    let response = client.get("https://api.exchangerate-api.com/v4/latest/CNY")
        .send()
        .await?
        .json::<ExchangeRate>()
        .await?;
    Ok(response)
}
