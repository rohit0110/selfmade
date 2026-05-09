use std::env;
use redis::Redis;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let port: u16 = args[1].parse().expect("Not valid port u16");
    let listener = TcpListener::bind(format!("127.0.0.1:{}",port)).await.unwrap();
    Redis::new(listener).run().await;
}