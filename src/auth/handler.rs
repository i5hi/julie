use std::str;
use serde_derive::{Deserialize, Serialize};

use crate::auth::client::{ClientAuth};
use crate::lib::error::S5ErrorKind;
use crate::auth::core;
 
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
pub async fn handle_put_basic(client:Result<ClientAuth,warp::Rejection>, auth_register: AuthRegister) -> Result<impl warp::Reply,warp::Rejection>{
    let client =  client?;
    let client = client.update_basic_auth(&auth_register.username, &auth_register.pass256).await;
    Ok(warp::reply::json(&client))
}
/// A warp filter for apikey auth
pub fn filter_apikey(key: String)-> Result<ClientAuth,warp::Rejection>{
    let client = match core::verify_apikey(&key){
        Some(client)=>client,
        None=> return Err(warp::reject::custom(S5ErrorKind::ApiKey))
    };
    Ok(client)
}

/// A warp filter for basic auth
pub fn filter_basic_auth(client: Result<ClientAuth,warp::Rejection>, basic_auth_encoded: String)-> Result<ClientAuth,warp::Rejection>{
    
    let client = client?;
    let trimmed = basic_auth_encoded.replace("Basic ", "");
    let status = core::verify_basic_auth(client.clone(), trimmed);

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
    let verify = core::verify_signature(client.clone(),&message, &signature);

    if verify {
        Ok(client)
    }
    else{
        return Err(warp::reject::custom(S5ErrorKind::Signature))
    }

}

