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
    let client = Client::new();

    // API 路由
    let rates = warp::path("api")
        .and(warp::path("rates"))
        .and_then(move || {
            let client = client.clone();
            async move {
                match fetch_rates(&client).await {
                    Ok(rates) => Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&rates)),
                    Err(_) => {
                        let error_response = ErrorResponse {
                            error: "Error fetching rates".to_string(),
                        };
                        Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&error_response))
                    }
                }
            }
        });


    // HTML 页面
    let index = warp::path::end()
        .map(|| warp::reply::html(include_str!("index.html")));

    // 启动服务器
    let routes = index.or(rates).recover(handle_rejection);
    println!("Starting server on http://0.0.0.0:3030");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030)).await;

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
