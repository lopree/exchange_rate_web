use warp::Filter;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::{time::sleep,task};
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Rates {
    rates: HashMap<String, f64>,
}

// 模拟从外部 API 获取汇率
async fn fetch_exchange_rates() -> Rates {
    // 模拟的汇率数据
    let mut rates = HashMap::new();
    rates.insert("USD".to_string(), 0.141);
    rates.insert("GBP".to_string(), 0.108);
    rates.insert("JPY".to_string(), 21.03);
    rates.insert("ARS".to_string(), 138.27);
    rates.insert("INR".to_string(), 11.83);
    rates.insert("TRY".to_string(), 4.81);
    rates.insert("HKD".to_string(), 1.09);
    rates.insert("TWD".to_string(), 4.5);
    rates.insert("SGD".to_string(), 0.184);

    Rates { rates }
}

// 后台任务，每隔一段时间（如1小时）更新汇率
async fn update_exchange_rates(cache: Arc<Mutex<Rates>>) {
    loop {
        // 获取新的汇率
        let rates = fetch_exchange_rates().await;

        // 更新缓存
        {
            let mut cache_lock = cache.lock().unwrap();
            *cache_lock = rates;
        } // 在这里释放锁

        // 休眠1小时后再更新
        sleep(Duration::from_secs(3600)).await;
    }
}

#[tokio::main]
async fn main() {
    // 提供静态文件服务，指向 "scr" 目录
    let static_files = warp::fs::dir("src");
    let rates_cache = Arc::new(Mutex::new(fetch_exchange_rates().await));

    // 启动后台任务定期更新汇率
    let cache_clone = rates_cache.clone();
    task::spawn(async move {
        update_exchange_rates(cache_clone).await;
    });

    // 创建汇率API路由，直接返回缓存的汇率
    let rates_api = warp::path("api")
        .and(warp::path("rates"))
        .map(move || {
            let cache_lock = rates_cache.lock().unwrap();
            warp::reply::json(&*cache_lock)
        });

    // 创建主页路由
    let hello = warp::path::end().map(|| {
        warp::reply::html(include_str!("src/index.html"))
    }).or(static_files);

    // 静态文件服务
    let static_files = warp::fs::dir("public");

    // 合并路由
    let routes = hello.or(rates_api).or(static_files);

    // 启动服务器
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
