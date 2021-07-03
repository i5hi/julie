use crate::auth::dto;
use tracing::{instrument};
use warp::{self, Filter};
use crate::lib::server;
use crate::storage::interface::{JulieStorage,JulieDatabase};
use crate::storage::sled::{SledDb};


/// Build a warp http router to serve all julie service apis.
#[instrument]
pub fn build() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let client_storage = SledDb::init(JulieDatabase::Client).unwrap();
    let service_storage = SledDb::init(JulieDatabase::Service).unwrap();

    let health = warp::path("julie")
        .and(warp::path("health"))
        .and(warp::get())
        .map(|| {
            format!("Rawfwaoof!")
        })
        .with(warp::trace::named("julie-health"));

    let put_basic = warp::path("julie")
        .and(warp::path("basic"))
        .and(warp::put())
        .and(warp::header::<String>("x-sats-api-key"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(with_backend(client_storage.clone()))
        .and_then(dto::handle_put_basic)
        .with(warp::trace::named("julie-put-basic"));
    
    let put_email = warp::path("julie")
        .and(warp::path("email"))
        .and(warp::put())
        .and(warp::header::<String>("x-sats-api-key"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(with_backend(client_storage.clone()))
        .and_then(dto::handle_put_email)
        .with(warp::trace::named("julie-put-email"));
    
    let post_email_callback = warp::path("julie")
        .and(warp::path("callback"))
        .and(warp::post())
        .and(warp::query::<dto::EmailCallbackQuery>())
        .and(with_backend(client_storage.clone()))
        .and_then(dto::handle_post_email_callback)
        .with(warp::trace::named("julie-post-email-callback"));
        
    let put_pubkey = warp::path("julie")
        .and(warp::path("pubkey"))
        .and(warp::put())
        .and(warp::header::<String>("x-sats-api-key"))
        .and(warp::header::<String>("Authorization"))
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .and(with_backend(client_storage.clone()))
        .and_then(dto::handle_put_pubkey)
        .with(warp::trace::named("julie-put-pubkey"));


    let get_token =warp::path("julie")
        .and(warp::path("token"))
        .and(warp::get())
        .and(warp::header::<String>("x-sats-api-key"))
        .and(warp::header::<String>("Authorization"))
        .and(warp::header::<String>("x-sats-client-signature"))
        .and(warp::header::<u64>("x-sats-timestamp"))
        .and(warp::query::<dto::ServiceQuery>())
        .and(with_backend(client_storage.clone()))
        .and(with_backend(service_storage.clone()))
        .and_then(dto::handle_get_token)
        .with(warp::trace::named("julie-get-token"));


    let julie_routes = health
        .or(put_basic)
        .or(put_email)
        .or(put_pubkey)
        .or(get_token)
        .or(post_email_callback)
        .recover(server::handle_rejection)
        .with(warp::trace::request());

    julie_routes
}


fn with_backend(storage: impl JulieStorage) -> impl Filter<Extract = (impl JulieStorage,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
