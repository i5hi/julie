use crate::auth::dto;
use tracing::{instrument};
use warp::{self, Filter};
use crate::lib::server;
use crate::storage::interface::{JulieStorage,JulieDatabase,StorageChoice};
use crate::storage::sled::{SledDb, init as init_sled};
use crate::storage::vault::{VaultStorage, init as init_vault};


/// Build a warp http router to serve all julie service apis.
#[instrument]
pub fn build(storage_choice: StorageChoice) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let (client_storage , _service_storage) = select_backend(storage_choice);

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
        .and(warp::header::<String>("x-sats-uid"))
        .and(warp::header::<String>("x-sats-api-key"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(with_backend(client_storage.clone_box()))
        .and_then(dto::handle_put_basic)
        .recover(server::handle_rejection)
        .with(warp::trace::named("julie-put-basic"));
    
    // let put_email = warp::path("julie")
    //     .and(warp::path("email"))
    //     .and(warp::put())
    //     .and(warp::header::<String>("x-sats-uid"))
    //     .and(warp::header::<String>("x-sats-api-key"))
    //     .and(warp::body::content_length_limit(1024 * 16))
    //     .and(warp::body::json())
    //     .and(with_backend(client_storage.clone_box()))
    //     .and_then(dto::handle_put_email)
    //     .with(warp::trace::named("julie-put-email"));

        
    // let put_pubkey = warp::path("julie")
    //     .and(warp::path("pubkey"))
    //     .and(warp::put())
    //     .and(warp::header::<String>("x-sats-uid"))
    //     .and(warp::header::<String>("x-sats-api-key"))
    //     .and(warp::body::content_length_limit(1024 * 64))
    //     .and(warp::body::json())
    //     .and(with_backend(client_storage.clone_box()))
    //     .and_then(dto::handle_put_pubkey)
    //     .with(warp::trace::named("julie-put-pubkey"));

    // let get_token =warp::path("julie")
    //     .and(warp::path("token"))
    //     .and(warp::get())
    //     .and(warp::header::<String>("x-sats-uid"))
    //     .and(warp::header::<String>("authorization"))
    //     .and(warp::header::<String>("x-sats-client-signature"))
    //     .and(warp::header::<u64>("x-sats-timestamp"))
    //     .and(warp::query::<dto::ServiceQuery>())
    //     .and(with_backend(client_storage.clone_box()))
    //     .and(with_backend(service_storage.clone_box()))
    //     .and_then(dto::handle_get_token)
    //     .with(warp::trace::named("julie-get-token"));


    let julie_routes = health
        .or(put_basic)
        .recover(server::handle_rejection)
        // .or(put_email)
        // .or(put_pubkey)
        // .or(get_token)
        // .or(post_email_callback)
        .with(warp::trace::request());

    julie_routes
}


fn with_backend(storage: Box<dyn JulieStorage>) -> impl Filter<Extract = (Box<dyn JulieStorage>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || storage.clone_box())
}


fn select_backend(choice: StorageChoice)->(Box<dyn JulieStorage>,Box<dyn JulieStorage>){
    match choice{
        StorageChoice::Sled=>{
            let client_storage = init_sled(JulieDatabase::Client).unwrap();
            let service_storage = init_sled(JulieDatabase::Service).unwrap();
            (Box::new(client_storage),Box::new(service_storage))
        }

        StorageChoice::Vault=>{
            let client_storage = init_vault(JulieDatabase::Client).unwrap();
            let service_storage = init_vault(JulieDatabase::Service).unwrap();
            (Box::new(client_storage),Box::new(service_storage))
        }
    }
}