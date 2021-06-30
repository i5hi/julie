#![allow(dead_code)]
// #![type_length_limit = "1422483"]
mod auth;
mod lib;

use std::str::FromStr;
use tracing_subscriber::fmt::format::FmtSpan;
use std::net::{SocketAddr};

#[tokio::main]
async fn main() {
    let _config = lib::config::JulieConfig::init();
    // println!("{:#?}",config.clone());
    // JSON doesnt read right
    // let mut _address = SocketAddr::from_str("127.0.0.1").unwrap();
    // address.set_port("3030".parse::<u16>().unwrap());

    tracing_subscriber::fmt()
    // Record an event when each span closes. This can be used to time our
    // routes' durations!
    .with_span_events(FmtSpan::CLOSE)
    .init();
    
    tracing::info!(".[|julie mfa daemon|].");
    lib::banner::print();
    warp::serve(auth::router::build()).run(([127,0,0,1],3030)).await
}

