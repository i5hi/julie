use serde::{Serialize, Deserialize};
use tracing::instrument;
use tracing::error;

// use std::convert::Infallible;
use thiserror::Error;


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
    #[error("No service named {0} registered.")]
    BadServiceIdentity(String),
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

impl warp::reject::Reject for S5ErrorKind {}

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
