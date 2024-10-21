use warp::Filter;
use tokio::sync::Mutex;
use tokio::task;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

type Rates = HashMap<String, f64>;
type Cache = Arc<Mutex<Rates>>;

async fn update_exchange_rates(cache: Cache) {
    loop {
        let new_rates = get_rates_from_api().await;

        let mut cache_lock = cache.lock().await;
        *cache_lock = new_rates;

        sleep(Duration::from_secs(10)).await;
    }
}

async fn get_rates_from_api() -> Rates {
    let mut rates = Rates::new();
    rates.insert("USD".to_string(), 1.0);
    rates.insert("GBP".to_string(), 0.108);
    rates.insert("JPY".to_string(), 21.03);
    rates.insert("ARS".to_string(), 138.27);
    rates.insert("INR".to_string(), 11.83);
    rates.insert("TRY".to_string(), 4.81);
    rates.insert("HKD".to_string(), 1.09);
    rates.insert("TWD".to_string(), 4.5);
    rates.insert("SGD".to_string(), 0.184);
    rates.insert("CNY".to_string(), 1.0);
    rates
}

#[tokio::main]
async fn main() {
    let cache: Cache = Arc::new(Mutex::new(get_rates_from_api().await));

    let cache_clone = cache.clone();
    task::spawn(async move {
        update_exchange_rates(cache_clone).await;
    });

    // API 路由
    let rates_api = warp::path("api")
        .and(warp::path("rates"))
        .and_then({
            let cache = cache.clone();
            move || {
                let cache = cache.clone();
                async move {
                    let rates = cache.lock().await;
                    Ok::<_, warp::Rejection>(warp::reply::json(&*rates))
                }
            }
        });

    // 静态文件路由
    let static_files = warp::fs::dir("src");

    // 主页路由
    let hello = warp::path::end()
        .map(|| warp::reply::html(include_str!("index.html")));

    // 使用 warp::any 来组合路由
    let routes = warp::any()
        .and(hello.or(rates_api).or(static_files));

    // 运行服务
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
