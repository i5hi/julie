use serde_derive::{Deserialize, Serialize};
use std::str;

use crate::storage::sled::{SledDb};
use crate::storage::interface::JulieClientStorage;

// use tracing::instrument;

use crate::auth::client::{AuthFactor, ClientAuth};
use crate::auth::service::{ServiceIdentity};

use crate::auth::controller;
use crate::lib::error::S5ErrorKind;
use crate::lib::server;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthBasic {
    username: String,
    pass256: String,
}
#[derive(Deserialize, Serialize,Debug, Clone)]
pub struct AuthEmail {
    email: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServiceQuery {
    service: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmailCallbackQuery {
    uid: String,
    token: String
}

#[derive(Deserialize, Serialize,Debug, Clone)]
pub struct AuthPublicKey {
    public_key: String,
}

#[derive(Deserialize, Serialize,Debug, Clone)]
pub struct AuthToken {
    token: String,
}

#[derive(Deserialize, Serialize,Debug, Clone)]
pub struct TotpEstablished {
    status: bool,
}


/// Handle a warp http request to update basic auth username and pass256.
pub async fn handle_put_basic(
    apikey: String,
    auth_basic: AuthBasic,
) -> Result<impl warp::Reply, warp::Rejection> {
    let storage = SledDb::init().unwrap();
    let client = filter_apikey(apikey)?;
    let _client = controller::update_basic_auth(storage, client, &auth_basic.username, &auth_basic.pass256);
    Ok(server::handle_response(warp::reply::json(&auth_basic)).await)
}

/// Handle a warp http request to update an email.
pub async fn handle_put_email(
    apikey: String,
    auth_email: AuthEmail,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut storage = SledDb::init().unwrap();
    let client = filter_apikey(apikey)?;
    let _client = controller::update_email(storage,client, &auth_email.email);
    Ok(server::handle_response(warp::reply::json(&auth_email)).await)
}
/// Handle a warp http request to handle email auth callback.
pub async fn handle_post_email_callback(
    callback_query: EmailCallbackQuery
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut storage = SledDb::init().unwrap();

    let client = match storage.read(&callback_query.uid){
        Ok(client)=>client,
        Err(_)=>return Err(warp::reject::custom(S5ErrorKind::UID))
    };

    if client.verify_email_token(callback_query.token){
        let verify = TotpEstablished {
            status: true,
        };
        Ok(server::handle_response(warp::reply::json(&verify)).await)

    }
    else{
        Err(warp::reject::custom(S5ErrorKind::Email))
    }
}
/// Handle a warp http request to update a public_key.
pub async fn handle_put_pubkey(
    apikey: String,
    encoded_basic: String,
    auth_pubkey: AuthPublicKey,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut storage = SledDb::init().unwrap();

    let client = filter_apikey(apikey);
    let client = filter_basic_auth(client, encoded_basic)?;
    let _client = controller::update_public_key(storage,client, &auth_pubkey.public_key);
    Ok(server::handle_response(warp::reply::json(&auth_pubkey)).await)
}
/// Handle a warp http request to get a totp_key.
pub async fn handle_get_totp_key(
    client: Result<ClientAuth, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let storage = SledDb::init().unwrap();
    let client = client?;
    let client = controller::update_totp_key(storage,client).unwrap();
    Ok(server::handle_response(warp::reply::json(&client)).await)
}
/// Handle a warp http request to post an otp to establish a totp_key.
pub async fn handle_post_totp(
    client: Result<ClientAuth, warp::Rejection>,
    otp: u64,
) -> Result<impl warp::Reply, warp::Rejection> {
    let client = client?;
    let verify = TotpEstablished {
        status: client.verify_totp(otp),
    };

    Ok(server::handle_response(warp::reply::json(&verify)).await)
}
/// Handle a warp http request to get a JWT token to a client for a given service
pub async fn handle_get_token(
    apikey: String,
    encoded_basic: String,
    signature: String,
    timestamp: u64,
    service: ServiceQuery,
) -> Result<impl warp::Reply, warp::Rejection> {

    let client = filter_apikey(apikey);
    let client = filter_basic_auth(client, encoded_basic);
    let client = filter_signature(client, signature, timestamp)?;
    
    match ServiceIdentity::init(&service.clone().service){
        Some(service_identity)=>{
            let token = AuthToken {
                token: client.issue_token(service_identity).unwrap(),
            };
        
            Ok(server::handle_response(warp::reply::json(&token)).await)
        },
        None=>{
            Err(warp::reject::custom(S5ErrorKind::BadServiceIdentity(service.clone().service)))
        },
    }


}

/// A warp filter for apikey auth UIDDD
/// /// FIIIXXXXX THISSS
pub fn filter_apikey(key: String) -> Result<ClientAuth, warp::Rejection> {
    let uid = "temp".to_string();
    let mut storage = SledDb::init().unwrap();
    let client = storage.read(&uid).unwrap();
    if client.verify_apikey(&uid, &key) {
        Ok(client)  
    }else{
        Err(warp::reject::custom(S5ErrorKind::ApiKey))
    }
}
/// A warp filter for basic auth
pub fn filter_basic_auth(
    client: Result<ClientAuth, warp::Rejection>,
    basic_auth_encoded: String,
) -> Result<ClientAuth, warp::Rejection> {
    let client = client?;
    let trimmed = basic_auth_encoded.replace("Basic ", "");
    let status = client.verify_basic_auth(trimmed);

    if status {
        Ok(client)
    } else {
        return Err(warp::reject::custom(S5ErrorKind::BasicAuth));
    }
}
/// A warp filter for signature auth
pub fn filter_signature(
    client: Result<ClientAuth, warp::Rejection>,
    signature: String,
    timestamp: u64,
) -> Result<ClientAuth, warp::Rejection> {
    let client = client?;
    let message = "timestamp=".to_string() + &timestamp.to_string();
    let mut verify = true;
    if client.factors.contains(&AuthFactor::Signature) || client.factors.contains(&AuthFactor::All) {
        verify = client.verify_signature( &message, &signature);
    }

    if verify {
        Ok(client)
    } else {
        return Err(warp::reject::custom(S5ErrorKind::Signature));
    }
}
/// A warp filter for totp auth
pub fn filter_totp(
    client: Result<ClientAuth, warp::Rejection>,
    otp: u64,
) -> Result<ClientAuth, warp::Rejection> {
    let client = client?;
    let mut verify = true;
    if client.factors.contains(&AuthFactor::Totp) || client.factors.contains(&AuthFactor::All) {
        verify = client.verify_totp(otp);
    }
    if verify {
        Ok(client)
    } else {
        return Err(warp::reject::custom(S5ErrorKind::BadTotp));
    }
}
