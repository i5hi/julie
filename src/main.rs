#![allow(dead_code)]
// #![type_length_limit = "1422483"]

mod auth;
mod lib;

use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    // Record an event when each span closes. This can be used to time our
    // routes' durations!
    .with_span_events(FmtSpan::CLOSE)
    .init();
    
    tracing::info!(".[|Julie on Duty|].");

    warp::serve(auth::router::build()).run(([127, 0, 0, 1], 3030)).await
}

