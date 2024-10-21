use warp::Filter;

#[tokio::main]
async fn main() {
    // 创建一个简单的路由，返回 "Hello, World"
    let hello = warp::path::end()
        .map(|| warp::reply::html("Hello, World"));

    // 启动服务器，监听所有地址的 3030 端口
    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030)).await; // 0.0.0.0 表示监听所有 IP
}
