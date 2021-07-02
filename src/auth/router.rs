use crate::auth::dto;
use tracing::{instrument};
use warp::{self, Filter};
use crate::lib::server;

/// Build a warp http router to serve all auth service apis.
#[instrument]
pub fn build() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let health = warp::path("auth")
        .and(warp::path("health"))
        .and(warp::get())
        .map(|| {
            format!("Rawfwaoof!")
        })
        .with(warp::trace::named("auth-health"));

    let put_basic = warp::path("auth")
        .and(warp::path("basic"))
        .and(warp::put())
        .and(warp::header::<String>("x-sats-api-key"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(dto::handle_put_basic)
        .with(warp::trace::named("auth-put-basic"));
        
    let put_pubkey = warp::path("auth")
        .and(warp::path("pubkey"))
        .and(warp::put())
        .and(warp::header::<String>("x-sats-api-key"))
        .and(warp::header::<String>("authorization"))
        // .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .and_then(dto::handle_put_pubkey)
        .with(warp::trace::named("auth-put-pubkey"));

    // let get_totp = auth_root.clone()
    //     .and(warp::path("totp"))
    //     .and(warp::path("key"))
    //     .and(warp::get())
    //     .and(warp::header::<String>("x-sats-api-key"))
    //     .map(dto::filter_apikey)
    //     .and(warp::header::<String>("Authorization"))
    //     .map(dto::filter_basic_auth)
    //     .and_then(dto::handle_get_totp_key)
    //     .recover(error::handle_rejection)
    //     .with(warp::trace::named("auth-get-totp"));

    // let post_totp = auth_root.clone()
    //     .and(warp::path("totp"))
    //     .and(warp::post())
    //     .and(warp::header::<String>("x-sats-api-key"))
    //     .map(dto::filter_apikey)
    //     .and(warp::header::<String>("Authorization"))
    //     .map(dto::filter_basic_auth)
    //     .and(warp::body::content_length_limit(1024 * 16))
    //     .and(warp::body::json())
    //     .and_then(dto::handle_post_totp)
    //     .recover(error::handle_rejection)
    //     .with(warp::trace::named("auth-post-totp"));

    let get_token =warp::path("auth")
        .and(warp::path("token"))
        .and(warp::get())
        .and(warp::header::<String>("x-sats-api-key"))
        .and(warp::header::<String>("Authorization"))
        .and(warp::header::<String>("x-sats-client-signature"))
        .and(warp::header::<u64>("x-sats-timestamp"))
        .and(warp::query::<dto::ServiceQuery>())
        .and_then(dto::handle_get_token)
        .with(warp::trace::named("auth-get-token"));


    let auth_routes = health
        .or(put_basic)
        .or(put_pubkey)
        .or(get_token)
        .recover(server::handle_rejection)
        .with(warp::trace::request());

    auth_routes
}
