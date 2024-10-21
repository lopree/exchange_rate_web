use warp::Filter;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
struct RateResponse {
    rates: std::collections::HashMap<String, f64>,
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
                    Ok(rates) => warp::reply::json(&rates),
                    Err(_) => {
                        let error_response = ErrorResponse {
                            error: "Error fetching rates".to_string(),
                        };
                        warp::reply::json(&error_response)
                    }
                }
            }
        });

    // HTML 页面
    let index = warp::path::end()
        .map(|| warp::reply::html(include_str!("index.html")));

    // 启动服务器
    let routes = index.or(rates);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030)).await;
}

async fn fetch_rates(client: &Client) -> Result<RateResponse, reqwest::Error> {
    let url = "https://api.exchangerate-api.com/v4/latest/USD"; // 示例 API
    let response = client.get(url).send().await?;
    let rates = response.json::<RateResponse>().await?;
    Ok(rates)
}
