use serde_derive::{Deserialize, Serialize};
use std::str;

// use tracing::instrument;

use crate::auth::client::{AuthLevel, ClientAuth};
use crate::auth::core;
use crate::lib::error::S5ErrorKind;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthBasic {
    username: String,
    pass256: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServiceQuery {
    service: String,
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

/// Handle a warp http request to register basic auth username and pass256.
pub async fn handle_put_basic(
    apikey: String,
    auth_basic: AuthBasic,
) -> Result<impl warp::Reply, warp::Rejection> {
    
    let client = filter_apikey(apikey)?;
    let _client = core::update_basic_auth(client, &auth_basic.username, &auth_basic.pass256);
    Ok(warp::reply::json(&auth_basic))
}
/// Handle a warp http request to register a public_key.
pub async fn handle_put_pubkey(
    apikey: String,
    encoded_basic: String,
    auth_pubkey: AuthPublicKey,
) -> Result<impl warp::Reply, warp::Rejection> {
    let client = filter_apikey(apikey);
    let client = filter_basic_auth(client, encoded_basic)?;
    let _client = core::update_public_key(client, &auth_pubkey.public_key);
    Ok(warp::reply::json(&auth_pubkey))
}
/// Handle a warp http request to get a totp_key.
pub async fn handle_get_totp_key(
    client: Result<ClientAuth, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let client = client?;
    let client = core::update_totp_key(client).unwrap();
    Ok(warp::reply::json(&client))
}
/// Handle a warp http request to post an otp to establish a totp_key.
pub async fn handle_post_totp(
    client: Result<ClientAuth, warp::Rejection>,
    otp: u64,
) -> Result<impl warp::Reply, warp::Rejection> {
    let client = client?;
    let verify = TotpEstablished {
        status: core::verify_totp(client.clone(), otp),
    };

    Ok(warp::reply::json(&verify))
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
    
    let token = AuthToken {
        token: core::issue_token(client, &service.service).unwrap(),
    };

    Ok(warp::reply::json(&token))
}

/// A warp filter for apikey auth
pub fn filter_apikey(key: String) -> Result<ClientAuth, warp::Rejection> {
    let client = match core::verify_apikey(&key) {
        Some(client) => client,
        None => return Err(warp::reject::custom(S5ErrorKind::ApiKey)),
    };
    Ok(client)
}
/// A warp filter for basic auth
pub fn filter_basic_auth(
    client: Result<ClientAuth, warp::Rejection>,
    basic_auth_encoded: String,
) -> Result<ClientAuth, warp::Rejection> {
    let client = client?;
    let trimmed = basic_auth_encoded.replace("Basic ", "");
    let status = core::verify_basic_auth(client.clone(), trimmed);

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
    if client.level == AuthLevel::Signature || client.level == AuthLevel::MultiFactor {
        verify = core::verify_signature(client.clone(), &message, &signature);
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
    if client.level == AuthLevel::Totp || client.level == AuthLevel::MultiFactor {
        verify = core::verify_totp(client.clone(), otp);
    }
    if verify {
        Ok(client)
    } else {
        return Err(warp::reject::custom(S5ErrorKind::BadTotp));
    }
}
