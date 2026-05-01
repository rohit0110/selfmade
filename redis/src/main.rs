use std::env;
use redis::Redis;

fn main() {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args[1].parse().expect("Not valid port u16");
    Redis::new(port).run();
}