use std::str;
use serde_derive::{Deserialize, Serialize};
use warp::{self };
use tracing::instrument;
use tracing::info;

use crate::auth::storage::{ClientAuth};
use crate::lib::error::S5ErrorKind;
use crate::lib::rsa;
use crate::lib::hash;
 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthRegister {
    username: String,
    pass256: String,
}

// #[derive(Deserialize, Serialize)]
// pub struct AuthConfig {
//     public_key: String,
// }
// #[derive(Deserialize, Serialize)]
// pub struct AuthToken {
//     token: String,
// }

/// Handle a warp http request to register basic auth username and pass256.
// #[instrument]
pub async fn handle_put_basic(client:Result<ClientAuth,warp::Rejection>, auth_register: AuthRegister) -> Result<impl warp::Reply,warp::Rejection>{
    let client =  client?;
    let client = client.update_basic_auth(&auth_register.username, &auth_register.pass256).await;
    Ok(warp::reply::json(&client))
}
/// A warp filter for apikey auth
pub fn filter_apikey(key: String)-> Result<ClientAuth,warp::Rejection>{
    let client = match ClientAuth::init(&key){
        Some(client)=>client,
        None=> return Err(warp::reject::custom(S5ErrorKind::ApiKey))
    };
    Ok(client)
}

/// A warp filter for basic auth
pub fn filter_basic_auth(client: Result<ClientAuth,warp::Rejection>, basic_auth_encoded: String)-> Result<ClientAuth,warp::Rejection>{
    
    let client = client?;

    let trimmed = basic_auth_encoded.replace("Basic ", "");
    let decoded_auth = str::from_utf8(&base64::decode(&trimmed).unwrap())
        .unwrap()
        .to_string();
    let parts = decoded_auth.split(":").collect::<Vec<&str>>();
    let username = parts[0];
    let pass256 = hash::sha256(&parts[1][0..64]);

    let mut status = false;   
    
    if &pass256 == &client.pass256 && username == &client.username {
        status = true
    } else {
        status = false
    }

    if status {
        Ok(client)
    }
    else {
        return Err(warp::reject::custom(S5ErrorKind::BasicAuth))
    }
}

/// A warp filter for signature auth
pub fn filter_signature(client: Result<ClientAuth,warp::Rejection>, signature: String, timestamp: u64)-> Result<ClientAuth,warp::Rejection>{
    
    let client =  client?;
    let message = "timestamp=".to_string() + &timestamp.to_string();
    let verify = rsa::verify(&message, &signature, &client.public_key);

    if verify {
        Ok(client)
    }
    else{
        return Err(warp::reject::custom(S5ErrorKind::Signature))
    }

}

