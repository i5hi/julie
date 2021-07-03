use serde_derive::{Deserialize, Serialize};
use std::str;
// use tracing::instrument;

use crate::storage::interface::{JulieStorage};

use crate::auth::client::{AuthFactor};

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
    name: String,
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
    uid: String,
    apikey: String,
    auth_basic: AuthBasic,
    client_storage: impl JulieStorage,
) -> Result<impl warp::Reply, warp::Rejection> {

    match controller::check_apikey(client_storage.clone(),apikey,uid.clone()){
        Ok(_)=>{
            if controller::update_basic_auth(client_storage, uid, &auth_basic.username, &auth_basic.pass256){
                return Ok(server::handle_response(warp::reply::json(&auth_basic)).await)
                
            }
            else{
                return Err(warp::reject::custom(S5ErrorKind::ServerError))
            }
        }
        Err(e)=>return Err(warp::reject::custom(e))
    }
  
}

/// Handle a warp http request to update an email.
pub async fn handle_put_email(
    uid: String,
    apikey: String,
    auth_email: AuthEmail,
    client_storage: impl JulieStorage,

) -> Result<impl warp::Reply, warp::Rejection> {
    match controller::check_apikey(client_storage.clone(),apikey,uid.clone()){
        Ok(_)=>{
            if controller::update_email(client_storage, uid, &auth_email.email){
                return Ok(server::handle_response(warp::reply::json(&auth_email)).await)
                
            }
            else{
                return Err(warp::reject::custom(S5ErrorKind::ServerError))
            }
        }
        Err(e)=>return Err(warp::reject::custom(e))
    }
}

pub async fn handle_put_pubkey(
    uid: String,
    apikey: String,
    auth_pubkey: AuthPublicKey,
    client_storage: impl JulieStorage,
) -> Result<impl warp::Reply, warp::Rejection> {
    match controller::check_apikey(client_storage.clone(),apikey,uid.clone()){
        Ok(_)=>{
            if controller::update_public_key(client_storage, uid, &auth_pubkey.public_key){
                return Ok(server::handle_response(warp::reply::json(&auth_pubkey)).await)
                
            }
            else{
                return Err(warp::reject::custom(S5ErrorKind::ServerError))
            }
        }
        Err(e)=>return Err(warp::reject::custom(e))
    }
}

pub async fn handle_get_token(
    uid: String,
    encoded_basic: String,
    signature: String,
    timestamp: u64,
    service: ServiceQuery,
    client_storage: impl JulieStorage,
    service_storage: impl JulieStorage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let client = controller::get_client(client_storage.clone(), uid.clone()).unwrap();
 
    for factor in client.factors.iter(){
        match factor{
            AuthFactor::Basic=>{
                match controller::check_basic_auth(client_storage.clone(),uid.clone(),encoded_basic.clone()){
                    Ok(_)=>{}
                    Err(e)=> return Err(warp::reject::custom(e))
                }
            }
            AuthFactor::Signature=>{
                match controller::check_signature(client_storage.clone(),uid.clone(),signature.clone(),timestamp.clone()){
                    Ok(_)=>{}
                    Err(e)=> return Err(warp::reject::custom(e))
                }
            }
            _=>{}
        }
    };
    

    match controller::get_service(service_storage.clone(), service.clone().name){
        Ok(service)=>{
            let token = service.issue_token(client.uid).unwrap();
            let auth_token = AuthToken {
                token: token
            };
            return Ok(server::handle_response(warp::reply::json(&auth_token)).await)
        },
        Err(_)=>{
            return Err(warp::reject::custom(S5ErrorKind::BadServiceIdentity(service.clone().name)))
        },
    };


}