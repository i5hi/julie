use std::str;

use crate::auth::client::{ClientAuth,AuthLevel};
use crate::auth::service::{ServiceIdentity};

use crate::lib::hash;
use crate::lib::rsa;
use crate::lib::totp;
use crate::lib::jwt;

use crate::lib::error::S5ErrorKind;

use oath::{HashType};


pub fn update_basic_auth(client: ClientAuth, username: &str, pass256: &str)->ClientAuth{
    client.clone().update("username",username);
    client.clone().update("pass256",&hash::sha256(pass256));
    let client = ClientAuth::read(&client.clone().uid).unwrap();
    match client.clone().level{
        AuthLevel::ApiKey=>client.update("level",AuthLevel::Basic.as_str()),
        _=>false,
    };
    client

}
pub fn update_public_key(client: ClientAuth, public_key: &str)->ClientAuth{
    client.clone().update("public_key",public_key);
    let client = ClientAuth::read(&client.clone().uid).unwrap();
    match client.clone().level{
        AuthLevel::ApiKey=>client.update("level",AuthLevel::Signature.as_str()),
        AuthLevel::Basic=>client.update("level",AuthLevel::Signature.as_str()),
        _=>false
    };
    client

}
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

pub fn verify_totp(client: ClientAuth, otp: u64)->bool{
    if totp::generate_otp(client.totp_key, HashType::SHA1)==otp {true}else{false}
}

pub fn issue_token(client: ClientAuth, service_name: &str)->Option<String>{
    let service = match ServiceIdentity::init(service_name){
        Some(service)=>service,
        None=>return None
    };
    let token = jwt::issue(client.uid, service.shared_secret, service.name, client.level.as_str().to_string());
    Some(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::hash::sha256;
    // use crate::lib::rsa;

    #[test]
    fn core_composite() { 
        let client_auth = ClientAuth::new();
        // admin gives client this new client_auth with an apikey

        // client then registers a username and password
        let username = "vmd";
        let password = "secret";
        // user must hash password
        let p256 = sha256(password);
        let pass256_expected =
            "2bb80d537b1da3e38bd30361aa855686bde0eacd7162fef6a25fe97bf527a25b".to_string();
        
        assert_eq!(p256.clone(), pass256_expected.clone());

        // user must encode uname:pass256 in base64
        let encoded = base64::encode(format!("{}:{}",username.clone(),p256.clone()).as_bytes());
        let encoded_expected = "dm1kOjJiYjgwZDUzN2IxZGEzZTM4YmQzMDM2MWFhODU1Njg2YmRlMGVhY2Q3MTYyZmVmNmEyNWZlOTdiZjUyN2EyNWI=";

        assert_eq!(encoded.clone(),encoded_expected.clone());
    
        // We store a hashed hash
        // p256 submitted by the user will differ from pass256 in the registered_client
        // this is because pass256 is the hashed version of the hashed password provided by the client. 
        // use verify_basic_auth which considers this when checking. do not check manually!

        let registered_client = update_basic_auth(client_auth.clone(), username, &p256.clone());

        let public_key = "-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAqkVu2BX3K2ZB+0F+dGor\necTfBM9GYqNxxn3tTMR61fEMBX0vPA5itSQcfh91pofKrvC65CZBnu71EElvo4hU\n9WTqjiaNJJDB3dxLbek2WEx57kCM7vewiwyosUdeBeqdxZX/Tp1xHEyB636J/L4R\nGan7XDGfWs47ZnnmR/CB13LuaHW08ej9WWNiy8UPs0LRzUZkwDNdnhec/N+j5GG0\nTBqwcgfaQDep4irtCuCQ9Q1pXrzgFEwqc0Qsr/F7V2cdJLvtLhG9CW6RZZdlNYbc\nIVNi+G7kVlSts7/81/EsjSAL8VMcvvj6CakBFzyUH4kgQRvlwVA3grL/7d39Wu5F\nBFPVm/40nSMnh28J0Sk/2E5Xt7xSQ9A43WM9mUNLSXkuEZbvMY09yzxzUZo9paPG\nbvKJY72tdmNvc2La0gaEhGlQf+7IDOs9uUBkOw0f+wyzM9bLNiQqLpeQ7cQH9rIT\nV4I+tbo4jEmI5vZwB2AImbsVXEn8z9OxV4TBqBciwi0jECcu5yh6b2cS/Gj7D+5x\nEGvtKO26/Iqpfrzf1Of7unF8DdYz8hZdGZ3Vs3di0apksmwbw7soNk6Q2R/c+c0X\nXneQKZxmDkvOPna1Zldx9n0WSloq+neDdwt0D9DyPORSad1/o1+grg6ksTylX72b\njO+9ZXTV/bfznGJI2ZojOGsCAwEAAQ==\n-----END PUBLIC KEY-----";
        let private_key = "-----BEGIN RSA PRIVATE KEY-----\n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\n-----END RSA PRIVATE KEY-----";
        
        let signatory_client = update_public_key(client_auth.clone(), public_key);
        let message = "timestamp=now";

        let signature = rsa::sign(message, private_key);

        let ready_client = verify_apikey(&registered_client.apikey).unwrap();
        let basic_status = verify_basic_auth(ready_client.clone(), encoded);
        let sign_status = verify_signature(ready_client.clone(), message, &signature);

        assert!(basic_status);
        assert!(sign_status);

        let service_name = "satoshipay";
        let service = ServiceIdentity::new(service_name);

        let token = issue_token(ready_client.clone(), service_name).unwrap();
        println!("Bearer {:#?}",token.clone());

        let verify = jwt::verify(token,service.shared_secret).unwrap();
        println!("{:#?}",verify);


    }
}