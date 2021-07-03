use std::str;
use std::time::SystemTime;

use crate::auth::service::{ServiceIdentity};
use crate::auth::client::{ClientAuth,AuthFactor, EMAIL_TOKEN_LIFETIME};

use crate::storage::interface::{JulieStorage, JulieDatabase};

use crate::lib::hash;
use crate::lib::aes;
use crate::lib::aes::{keygen,Encoding};
use crate::lib::error::S5ErrorKind;


pub fn update_basic_auth(mut storage: impl JulieStorage, client: ClientAuth, username: &str, password: &str)->ClientAuth{
    
    let mut updated = storage.read(JulieDatabase::Client,&client.clone().uid).unwrap().0;
    updated.username = username.to_string();
    updated.pass512 = hash::salted512(password,&updated.salt);
    if updated.factors.contains(&AuthFactor::Basic){
        // do nothing
    }
    else{
        updated.factors.push(AuthFactor::Basic);
    }
    if storage.update(JulieDatabase::Client,(updated.clone(),ServiceIdentity::dummy())).unwrap(){
        updated.clone()
    }
    else{
        client.clone()
    }
}
pub fn update_email(mut storage: impl JulieStorage, client: ClientAuth, email: &str)->ClientAuth{
    let mut updated = storage.read(JulieDatabase::Client,&client.clone().uid).unwrap().0;
    updated.email = email.to_string();
    if updated.factors.contains(&AuthFactor::Email){
        // do nothing
    }
    else{
        updated.factors.push(AuthFactor::Email);
    }
   
    if storage.update(JulieDatabase::Client,(updated.clone(),ServiceIdentity::dummy())).unwrap(){
        updated.clone()
    }
    else{
        client.clone()
    }
    

}
pub fn update_public_key(mut storage: impl JulieStorage, client: ClientAuth, public_key: &str)->ClientAuth{
    let mut updated = storage.read(JulieDatabase::Client,&client.clone().uid).unwrap().0;
    updated.public_key = public_key.to_string();
    if updated.factors.contains(&AuthFactor::Signature){
        // do nothing
    }
    else{
        updated.factors.push(AuthFactor::Signature);
    }
   
    if storage.update(JulieDatabase::Client,(updated.clone(),ServiceIdentity::dummy())).unwrap(){
        updated.clone()
    }
    else{
        client.clone()
    }

}
pub fn update_totp_key(mut storage: impl JulieStorage, client: ClientAuth)->Result<ClientAuth,S5ErrorKind>{
    let mut updated = storage.read(JulieDatabase::Client,&client.clone().uid).unwrap().0;

    if updated.factors.contains(&AuthFactor::Totp){
        return Err(S5ErrorKind::TotpKeyEstablished)
    }
    else{
        updated.totp_key = keygen(Encoding::Base32);
    }

    if storage.update(JulieDatabase::Client,(updated.clone(),ServiceIdentity::dummy())).unwrap(){
        Ok(updated.clone())
    }
    else{
        Ok(client.clone())
    }

}
pub fn update_email_status(mut storage: impl JulieStorage, client: ClientAuth)->Result<bool,S5ErrorKind>{
    let mut updated = storage.read(JulieDatabase::Client,&client.clone().uid).unwrap().0;

    let expiry = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs() + EMAIL_TOKEN_LIFETIME,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let token = aes::keygen(aes::Encoding::Hex);

    updated.email_expiry = expiry;
    updated.email_token = token;

    if storage.update(JulieDatabase::Client,(updated.clone(),ServiceIdentity::dummy())).unwrap(){
        Ok(true)
    }
    else{
        Ok(false)
    }
    

}
pub fn establish_totp(mut storage: impl JulieStorage, client: ClientAuth)->Result<ClientAuth,S5ErrorKind>{
    let mut updated = storage.read(JulieDatabase::Client,&client.clone().uid).unwrap().0;

    if updated.factors.contains(&AuthFactor::Totp){
        return Err(S5ErrorKind::TotpKeyEstablished)
    }
    else{
        updated.factors.push(AuthFactor::Totp);
    }

    if storage.update(JulieDatabase::Client,(updated.clone(),ServiceIdentity::dummy())).unwrap(){
        Ok(updated.clone())
    }
    else{
        Ok(client.clone())
    }

}
/**
 * 
 * DONT NEED STORAGE
 * 
 * MOVE ME
 * 
 */
/// Since this initializes the auth process, we return a ClientAuth, where other verification functions return bool


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::hash::sha256;
    use crate::lib::rsa;
    use crate::storage::sled::{SledDb};
    use crate::lib::jwt;
    use oath::{HashType};
    use crate::lib::totp;

    #[test]
    fn core_composite() { 
        let client_auth = ClientAuth::new();
        // let config = SledConfig{
        //     db: "client".to_string()
        // };
        let mut client_storage = SledDb::init(JulieDatabase::Client).unwrap();
        assert!(client_storage.create(JulieDatabase::Client,(client_auth.clone(),ServiceIdentity::dummy())).unwrap());

        // admin gives client this new client_auth with an apikey
        // client then registers a username and password
        let username = "vmd";
        let password = "secret";
        // user must hash password
        let p256 = sha256(password);
        let p256_expected =
            "2bb80d537b1da3e38bd30361aa855686bde0eacd7162fef6a25fe97bf527a25b".to_string();
        
        assert_eq!(p256.clone(), p256_expected.clone());

        // user must encode uname:pass256 in base64
        let encoded = base64::encode(format!("{}:{}",username.clone(),p256.clone()).as_bytes());
        let encoded_expected = "dm1kOjJiYjgwZDUzN2IxZGEzZTM4YmQzMDM2MWFhODU1Njg2YmRlMGVhY2Q3MTYyZmVmNmEyNWZlOTdiZjUyN2EyNWI=";

        assert_eq!(encoded.clone(),encoded_expected.clone());
    
        // We store a hashed hash
        // p256 submitted by the user will will be salted and stored as pass512

        let registered_client = update_basic_auth(client_storage.clone(),client_auth.clone(), username, &p256.clone());

        let public_key = "-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAqkVu2BX3K2ZB+0F+dGor\necTfBM9GYqNxxn3tTMR61fEMBX0vPA5itSQcfh91pofKrvC65CZBnu71EElvo4hU\n9WTqjiaNJJDB3dxLbek2WEx57kCM7vewiwyosUdeBeqdxZX/Tp1xHEyB636J/L4R\nGan7XDGfWs47ZnnmR/CB13LuaHW08ej9WWNiy8UPs0LRzUZkwDNdnhec/N+j5GG0\nTBqwcgfaQDep4irtCuCQ9Q1pXrzgFEwqc0Qsr/F7V2cdJLvtLhG9CW6RZZdlNYbc\nIVNi+G7kVlSts7/81/EsjSAL8VMcvvj6CakBFzyUH4kgQRvlwVA3grL/7d39Wu5F\nBFPVm/40nSMnh28J0Sk/2E5Xt7xSQ9A43WM9mUNLSXkuEZbvMY09yzxzUZo9paPG\nbvKJY72tdmNvc2La0gaEhGlQf+7IDOs9uUBkOw0f+wyzM9bLNiQqLpeQ7cQH9rIT\nV4I+tbo4jEmI5vZwB2AImbsVXEn8z9OxV4TBqBciwi0jECcu5yh6b2cS/Gj7D+5x\nEGvtKO26/Iqpfrzf1Of7unF8DdYz8hZdGZ3Vs3di0apksmwbw7soNk6Q2R/c+c0X\nXneQKZxmDkvOPna1Zldx9n0WSloq+neDdwt0D9DyPORSad1/o1+grg6ksTylX72b\njO+9ZXTV/bfznGJI2ZojOGsCAwEAAQ==\n-----END PUBLIC KEY-----";
        let private_key = "-----BEGIN RSA PRIVATE KEY-----\n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\n-----END RSA PRIVATE KEY-----";
        
        let signatory_client = update_public_key(client_storage.clone(),registered_client, public_key);
        let message = "timestamp=now";

        let signature = rsa::sign(message, private_key);

        assert!(signatory_client.verify_apikey(&client_auth.clone().uid, &client_auth.clone().apikey));
        println!("{:?},{}",signatory_client.clone(),encoded.clone());
        assert!(signatory_client.verify_basic_auth(encoded));
        assert!(signatory_client.verify_signature( message, &signature));

        let service_name = "satoshipay";
        let shared_secret = keygen(Encoding::Hex);

        let service = ServiceIdentity::new(service_name,&shared_secret);
        let mut service_storage = SledDb::init(JulieDatabase::Service).unwrap();
        assert!(service_storage.create(JulieDatabase::Service, (ClientAuth::new(),service.clone())).unwrap());

        let token = service.issue_token(signatory_client.clone().uid).unwrap();
        println!("Bearer {:#?}",token.clone());

        let verify = jwt::verify(token,service.clone().shared_secret).unwrap();
        println!("{:#?}",verify);

        // Upgrade client to mfa
        let mfa_client = update_totp_key(client_storage.clone(),signatory_client.clone()).unwrap();

        let otp = totp::generate_otp(mfa_client.clone().totp_key, HashType::SHA1);
        assert!(mfa_client.verify_totp(otp));

        let mfa_client = client_storage.read(JulieDatabase::Client,&mfa_client.uid).unwrap().0;
        let token = service.issue_token(mfa_client.clone().uid).unwrap();
        println!("Bearer {:#?}",token.clone());

        let verify = jwt::verify(token,service.clone().shared_secret).unwrap();
        println!("{:#?}",verify);

        println!("{:#?}",mfa_client.clone());

        // Comment out the following if you want a user to persist for bash testing
        assert!(client_storage.delete(&mfa_client.uid).unwrap());
        assert!(service_storage.delete(&service.clone().name).unwrap());
        ()


    }

    #[test] #[ignore]
    fn core_email_composite(){
        // let config = SledConfig{
        //     db: "client".to_string()
        // };
        let mut storage = SledDb::init(JulieDatabase::Client).unwrap();
        let client_auth = ClientAuth::new();
        // admin gives client this new client_auth with an apikey
        // client then registers a username and password
        let email = "vishalmenon.92@gmail.com";
        let client_auth = update_email(storage.clone(),client_auth.clone(), email);
        client_auth.send_email_token();
        assert!(storage.delete(&client_auth.uid).unwrap())
    }
}