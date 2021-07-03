#![allow(dead_code)]
// #![type_length_limit = "1422483"]
mod lib;
mod auth;
mod storage;

use std::env;
// use std::str::FromStr;
use tracing_subscriber::fmt::format::FmtSpan;
// use std::net::{SocketAddr};
use tracing_appender;
use crate::lib::database::STORAGE_ROOT;

#[tokio::main]
async fn main() {
    let _config = lib::config::JulieConfig::init();
    // println!("{:#?}",config.clone());
    // JSON doesnt read right
    // let mut _address = SocketAddr::from_str("127.0.0.1").unwrap();
    // address.set_port("3030".parse::<u16>().unwrap());
    let root_dir = format!("{}/{}", env::var("HOME").unwrap(), STORAGE_ROOT);

    let file_appender = tracing_appender::rolling::hourly(root_dir, "daemon.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    
    tracing::info!(".[|julie mfa daemon|].");
    lib::banner::print();
    warp::serve(auth::router::build()).run(([127,0,0,1],3030)).await
}

