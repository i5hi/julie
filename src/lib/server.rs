use crate::lib::error::{S5Error,S5ErrorKind};
// use crate::lib::rsa;
use tracing::instrument;
use warp::{http::StatusCode, Rejection, Reply};


#[instrument]
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let code;
    let mut message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    }else if let Some(_) = err.find::<warp::reject::InvalidQuery>() {
        code = StatusCode::BAD_REQUEST;
        message = "Bad Query Params";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    }else if let Some(header) = err.find::<warp::reject::MissingHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = header.name();
    }else if let Some(header) = err.find::<warp::reject::InvalidHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = header.name();
    }else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
        code = StatusCode::BAD_REQUEST;
        message = "Payload Too Large";
    }
     else if let Some(e) = err.find::<S5ErrorKind>() {
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
            S5ErrorKind::BadServiceIdentity(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "No such service registered.";
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
            S5ErrorKind::UID => {
                code = StatusCode::UNAUTHORIZED;
                message = "Bad UID.";
            }
            S5ErrorKind::Email => {
                code = StatusCode::UNAUTHORIZED;
                message = "Email token invalid or expired.";
            }
            S5ErrorKind::NotInDatabase => {
                code = StatusCode::NOT_FOUND;
                message = "Resource not found.";
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
    let coded = warp::reply::with_status(json,code);
    let timestamped = warp::reply::with_header(coded, "x-timestamp",0);
    let signed = warp::reply::with_header(timestamped, "x-julie-signature", "rawfwoof");
    
    Ok(signed)
}


pub async fn handle_response(response: impl warp::Reply) -> impl warp::Reply {
    let coded = warp::reply::with_status(response,StatusCode::OK);
    let timestamped = warp::reply::with_header(coded, "x-timestamp",0);
    let signed = warp::reply::with_header(timestamped, "x-julie-signature", "rawfwoof");
    
    signed

}
