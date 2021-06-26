use warp::{self,Filter};
use crate::auth::handler;
use crate::lib::error;
use tracing::{instrument,Level};

/// Build a warp http router to serve all auth service apis.
#[instrument]
pub fn build()-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    
    let auth_root = warp::path("auth");
    let basic = auth_root
        .and(warp::path("basic"))
        .and(warp::put())
        .and(warp::header::<String>("x-sats-api-key"))
        .map(handler::filter_apikey)
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handler::handle_put_basic)
        .recover(error::handle_rejection)
        .with(warp::trace::named("auth-basic"));
        
    let auth_routes = basic;
        // .with(warp::trace::request());

    auth_routes
}
