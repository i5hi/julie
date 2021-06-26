use crate::lib::error::{S5ErrorKind};

use base64::{DecodeError};
use std::time::{ SystemTime};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::ErrorKind};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    exp: u64,
    uid: String
}

pub fn issue(uid: String,key: String)->String{
    let my_claims = Claims {
        iss: String::from("satsbank.io"),
        aud: String::from("client.satsbank.io"),
        exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        uid: uid
    };
    encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key.as_str().as_ref())).unwrap()
}
pub fn verify(token:String, key:String)->Result<String,S5ErrorKind>{ // return uid if true
    match decode::<Claims>(&token, &DecodingKey::from_secret(key.as_str().as_ref()), &Validation::default()){
       Ok(c) => Ok(c.claims.uid),
       Err(err) => match *err.kind() {
           ErrorKind::InvalidToken => Err(S5ErrorKind::JwtInvalid),
           ErrorKind::Base64(DecodeError::InvalidLength) => Err(S5ErrorKind::JwtInvalid),
           ErrorKind::Base64(DecodeError::InvalidLastSymbol(_usize,_u8)) => Err(S5ErrorKind::JwtInvalid),
           ErrorKind::Base64(DecodeError::InvalidByte(_usize,_u8)) => Err(S5ErrorKind::JwtInvalid),
           ErrorKind::ExpiredSignature=>Err(S5ErrorKind::JwtExpired),
           _ => panic!("{:#?}",err),
       },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_jwt() {
        let key = String::from("superseecret");
        let uid  = String::from("s5idVishalMenon");
        println!("JWT: {}",issue(uid,key));
    }
    #[test]
    fn verify_jwt_invalid(){
        let key = String::from("superseecret");
        let token = String::from("yJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJzYXRzYmFuay5pbyIsImF1ZCI6ImNsaWVudC5zYXRzYmFuay5pbyIsImV4cCI6MTYxNzk0OTk2MCwidWlkIjoiczVpZFZpc2hhbE1lbm9uIn0.0dbAs07HYbMPBRCwVq5MXlHHYYR4jvppe0KEBlSlIyk");
        // println!("DECODED: {}",verify(token, key).unwrap());

        match verify(token, key){
            Ok(_)=> panic!("FAILED. SHOULD HAVE ERRORED FOR INVALID TOKEN!!!"),
            Err(e)=>{
                let expected = S5ErrorKind::JwtInvalid;
                assert_eq!(e,expected)
            }
        }

    }
    #[test]
    fn verify_jwt_expired(){
        let key = String::from("superseecret");
        let token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJzYXRzYmFuay5pbyIsImF1ZCI6ImNsaWVudC5zYXRzYmFuay5pbyIsImV4cCI6MTYxNzk0OTk2MCwidWlkIjoiczVpZFZpc2hhbE1lbm9uIn0.0dbAs07HYbMPBRCwVq5MXlHHYYR4jvppe0KEBlSlIyk");
        // println!("DECODED: {}",verify(token, key).unwrap());

        match verify(token, key){
            Ok(_)=> panic!("FAILED. SHOULD HAVE ERRORED FOR EXPIRED TOKEN!!!"),
            Err(e)=>{
                let expected = S5ErrorKind::JwtExpired;
                assert_eq!(e,expected)
            }
        }

    }
}
