use serde::{Serialize, Deserialize};
use tracing::instrument;
use tracing::error;

// use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};


#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum S5ErrorKind{
    #[error("Api Key Sucks!")]
    ApiKey,
    #[error("Signature Sucks!")]
    Signature,
    #[error("Basic Auth Sucks!")]
    BasicAuth,
    #[error("RSA Public Key Sucks!")]
    PublicKey,
    #[error("Totp Sucks!")]
    BadTotp,
    #[error("Totp Key Established!")]
    TotpKeyEstablished,
    #[error("!!!Internal Server Error!!!")]
    ServerError,
    #[error("JWT Sucks!")]
    JwtInvalid,
    #[error("JWT Expired!")]
    JwtExpired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S5Error{
    pub message: String
}


impl S5Error{
    pub fn new(message: &str)->Self{
        S5Error{
            message: message.to_string()
        }
    }
}

impl warp::reject::Reject for S5ErrorKind {}

#[instrument]
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Rejection> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<S5ErrorKind>() {
        match e {
            S5ErrorKind::ApiKey => {
                code = StatusCode::UNAUTHORIZED;
                message = "Api Key Sucks!";
            }
            S5ErrorKind::Signature => {
                code = StatusCode::UNAUTHORIZED;
                message = "Signature Sucks!";
            }
            S5ErrorKind::BasicAuth => {
                code = StatusCode::UNAUTHORIZED;
                message = "Basic Auth Sucks!";
            }
            S5ErrorKind::PublicKey => {
                code = StatusCode::BAD_REQUEST;
                message = "Public Key Sucks!";
            }
            S5ErrorKind::BadTotp => {
                code = StatusCode::UNAUTHORIZED;
                message = "Bad TOTP";
            }
            S5ErrorKind::TotpKeyEstablished => {
                code = StatusCode::CONFLICT;
                message = "Totp Key Already Established.";
            }
            S5ErrorKind::ServerError => {
                code = StatusCode::UNAUTHORIZED;
                message = "Internal Server Error";
            }
            S5ErrorKind::JwtInvalid => {
                code = StatusCode::UNAUTHORIZED;
                message = "Invalid  Token";
            }
            S5ErrorKind::JwtExpired => {
                code = StatusCode::UNAUTHORIZED;
                message = "Expired Token";
            }
     
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        println!("!!HOLY MOLY!!\n\n\n{:#?}\n\n\n",err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }
    // error!("#$)(#$#)$(#*()OIakku PLSS!!)(*@)(#@#(_@");
    
    let json = warp::reply::json(&S5Error::new(&message));

    Ok(warp::reply::with_status(json, code))
}

