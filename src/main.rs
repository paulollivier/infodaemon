use sysinfo::{System, SystemExt};
use std::collections::HashMap;
use std::env;
use warp::Filter;
use serde::Serialize;

#[derive(Serialize)]
struct SysInfo {
    env: HashMap<String, String>
}

impl SysInfo {
    fn new() -> Self {
        let mut info = SysInfo {
            env: HashMap::new()
        };
        for (k, v) in env::vars() {
            info.env.insert(k, v);
        }
        info
    }
}

#[tokio::main]
async fn main() {
    let info = warp::path!("info").map(|| {
        let info = SysInfo::new();
        warp::reply::json(&info)
    });
    warp::serve(info).run(([127,0,0,1], 3030)).await;
}