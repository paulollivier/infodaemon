use std::collections::HashMap;
use std::env;

use warp::Filter;

use sysinfo::{DiskExt, System, SystemExt};

use serde::Serialize;

#[derive(Serialize)]
struct DiskInfo {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
}

#[derive(Serialize)]
struct SysInfo {
    env: HashMap<String, String>,
    disks: Vec<DiskInfo>,
}

impl SysInfo {
    fn new() -> Self {
        let mut info = SysInfo {
            env: HashMap::new(),
            disks: Vec::new(),
        };
        let mut sysinfo = System::new_all();
        sysinfo.refresh_all();
        for disk in sysinfo.get_disks() {
            info.disks.push(DiskInfo {
                name: String::from(
                    disk.get_name()
                        .to_str()
                        .expect("could not unwrap disk name to str"),
                ),
                mount_point: String::from(
                    disk.get_mount_point()
                        .to_str()
                        .expect("could not unwrap disk mount point to str"),
                ),
                total_space: disk.get_total_space(),
                available_space: disk.get_available_space(),
            });
        }
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
    println!("Launching server...");
    warp::serve(info).run(([0, 0, 0, 0], 3030)).await;
    println!("Quitting");
}
