#![allow(non_upper_case_globals)]

use shadow_rs::shadow;
use std::collections::HashMap;
use std::error::Error;
use clap::{App, Arg};

mod rbi_service;
mod rvps_handlers;
mod cache;

shadow!(build);

fn get_kvtype(m: &HashMap<String, String>) -> String {
    match m.get("kvtype") {
        None => "simple".to_string(),
        Some(s) => s.clone()
    }
}

fn get_address(m: &HashMap<String, String>) -> String {
    let port = match m.get("port") {
        None => "7654".to_string(),
        Some(s) => s.clone(),
    };

    let addr = match m.get("address") {
        None => "[::1]".to_string(),
        Some(s) => s.clone(),
    };

    format!("{}:{}", addr, port)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let version = format!(
        "v{}\ncommit: {}\nbuildtime: {}",
        build::PKG_VERSION,
        build::COMMIT_HASH,
        build::BUILD_TIME
    );
    println!("RBI info: {}", version);

    let matches = App::new("RBI")
        .version(version.as_str())
        .long_version(version.as_str())
        .author("Inclavare-Containers Team")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("config")
                .help("Path to the config")
                .takes_value(true),
        )
        .get_matches();

    let config_path = match matches.is_present("config") {
        true => matches.value_of("config").unwrap().to_string(),
        false => "config/config.toml".to_string(),
    };

    let mut cfg = config::Config::default();
    cfg
        .merge(config::File::with_name(config_path.as_str()))?
        .merge(config::Environment::with_prefix("APP"))?;
    let settings = cfg
        .try_into::<HashMap<String, String>>()?;

    let kv_type = get_kvtype(&settings);
    let kv_store = cache::new(&kv_type)?;
    let grpc_addr = get_address(&settings);
    println!("Listen gRPC server addr: {}", grpc_addr);

    let rbi_server = rbi_service::rbi_service_grpc::server(&grpc_addr, kv_store);

    tokio::join!(rbi_server).0
}
