use std::str;

use crate::auth::client::{ClientAuth};
use crate::lib::hash;
use crate::lib::rsa;
use crate::lib::totp;
use oath::{HashType};


/// Since this initializes the auth process, we return a ClientAuth, where other verification functions return bool
pub fn verify_apikey(apikey: &str)->Option<ClientAuth>{
    ClientAuth::init(&apikey)
}

pub fn verify_basic_auth(client: ClientAuth, basic_auth_encoded: String)->bool{
    let decoded_auth = str::from_utf8(&base64::decode(&basic_auth_encoded).unwrap())
        .unwrap()
        .to_string();
    let parts = decoded_auth.split(":").collect::<Vec<&str>>();
    let username = parts[0];
    let pass256 = hash::sha256(&parts[1][0..64]);

    
    if &pass256 == &client.pass256 && username == &client.username {
         true
    } else {
        false
    }
}

pub fn verify_signature(client: ClientAuth, message: &str, signature: &str)->bool{
    rsa::verify(&message, &signature, &client.public_key)
}

pub fn verify_otp(client: ClientAuth, otp: u64)->bool{
    if totp::generate_otp(client.totp_key, HashType::SHA1)==otp {true}else{false}
}