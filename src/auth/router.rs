use crate::auth::handler;
use crate::lib::error;
use tracing::{instrument};
use warp::{self, Filter};

/// Build a warp http router to serve all auth service apis.
#[instrument]
pub fn build() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let auth_root = warp::path("auth");

    let put_basic = auth_root
        .and(warp::path("basic"))
        .and(warp::put())
        .and(warp::header::<String>("x-sats-api-key"))
        .map(handler::filter_apikey)
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handler::handle_put_basic)
        .recover(error::handle_rejection)
        .with(warp::trace::named("auth-put-basic"));
        
    let put_pubkey = auth_root
        .and(warp::path("pubkey"))
        .and(warp::put())
        .and(warp::header::<String>("x-sats-api-key"))
        .map(handler::filter_apikey)
        .and(warp::header::<String>("Authorization"))
        .map(handler::filter_basic_auth)
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handler::handle_put_pubkey)
        .recover(error::handle_rejection)
        .with(warp::trace::named("auth-put-pubkey"));

    let get_totp = auth_root
        .and(warp::path("totp"))
        .and(warp::path("key"))
        .and(warp::get())
        .and(warp::header::<String>("x-sats-api-key"))
        .map(handler::filter_apikey)
        .and(warp::header::<String>("Authorization"))
        .map(handler::filter_basic_auth)
        .and_then(handler::handle_get_totp_key)
        .recover(error::handle_rejection)
        .with(warp::trace::named("auth-get-totp"));

    let post_totp = auth_root
        .and(warp::path("totp"))
        .and(warp::post())
        .and(warp::header::<String>("x-sats-api-key"))
        .map(handler::filter_apikey)
        .and(warp::header::<String>("Authorization"))
        .map(handler::filter_basic_auth)
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handler::handle_post_totp)
        .recover(error::handle_rejection)
        .with(warp::trace::named("auth-post-totp"));

    let get_token = auth_root
        .and(warp::path("token"))
        .and(warp::get())
        .and(warp::header::<String>("x-sats-api-key"))
        .map(handler::filter_apikey)
        .and(warp::header::<String>("Authorization"))
        .map(handler::filter_basic_auth)
        .and(warp::header::<u64>("x-sats-totp"))
        .map(handler::filter_totp)
        .and(warp::header::<String>("x-sats-signature"))
        .and(warp::header::<u64>("x-sats-timestamp"))
        .map(handler::filter_signature)
        .and(warp::query::<String>())
        .and_then(handler::handle_get_token)
        .recover(error::handle_rejection)
        .with(warp::trace::named("auth-post-totp"));

    let health =auth_root
    .and(warp::path("health"))
    .and(warp::get())
    .and(warp::header("user-agent"))
    .map(|agent: String| {
        format!("Hello agent {}", agent)
    })
    .recover(error::handle_rejection)
    .with(warp::trace::named("health"));

    let auth_routes = put_basic
        .or(put_pubkey)
        .or(get_totp)
        .or(post_totp)
        .or(get_token)
        .or(health)
        .with(warp::trace::request());

    auth_routes
}
