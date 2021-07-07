use std::str;
use std::time::SystemTime;

use crate::auth::service::{ServiceIdentity};
use crate::auth::client::{ClientAuth,AuthFactor, EMAIL_TOKEN_LIFETIME};

use crate::storage::interface::{JulieStorage, JulieDatabase, JulieDatabaseItem};

use crate::lib::hash;
use crate::lib::aes;
use crate::lib::aes::{keygen,Encoding};
use crate::lib::error::S5ErrorKind;

// RETURN A RESULT PLEASE
pub fn update_basic_auth(mut storage: Box<dyn JulieStorage>, uid: String, username: &str, password: &str)->bool{
    // the following will proceed with a new client - BAD
    let mut updating = match storage.read(JulieDatabase::Client,&uid).unwrap(){
            JulieDatabaseItem::Client(client)=>client,
            JulieDatabaseItem::Service(_)=>{panic!("OH NO! LOOK AT WHAT YOUVE DONE!")},
    };
    updating.username = username.to_string();
    updating.pass512 = hash::salted512(password,&updating.salt);
    if updating.factors.contains(&AuthFactor::Basic){
        // do nothing
    }
    else{
        updating.factors.push(AuthFactor::Basic);
    }
    if storage.update(JulieDatabaseItem::Client(updating.clone())).unwrap(){
        true
    }
    else{
        false
    }
}

// pub fn update_email(mut storage: impl JulieStorage, uid: String, email: &str)->bool{
//     let mut updating = match storage.read(JulieDatabase::Client,&uid).unwrap(){
//         JulieDatabaseItem::Client(client)=>client,
//         JulieDatabaseItem::Service(_)=>{panic!("OH NO! LOOK AT WHAT YOUVE DONE!")},
//     };

//     updating.email = email.to_string();
//     if updating.factors.contains(&AuthFactor::Email){
//         // do nothing
//     }
//     else{
//         updating.factors.push(AuthFactor::Email);
//     }
   
//     if storage.update(JulieDatabaseItem::Client(updating.clone())).unwrap(){
//         true
//     }
//     else{
//         false
//     }
    

// }
// pub fn update_public_key(mut storage: impl JulieStorage, uid: String, public_key: &str)->bool{
//     let mut updating = match storage.read(JulieDatabase::Client,&uid).unwrap(){
//         JulieDatabaseItem::Client(client)=>client,
//         JulieDatabaseItem::Service(_)=>{panic!("OH NO! LOOK AT WHAT YOUVE DONE!")},
//     };
    
//     updating.public_key = public_key.to_string();
//     if updating.factors.contains(&AuthFactor::Signature){
//         // do nothing
//     }
//     else{
//         updating.factors.push(AuthFactor::Signature);
//     }
   
//     if storage.update(JulieDatabaseItem::Client(updating.clone())).unwrap(){
//         true
//     }
//     else{
//         false
//     }

// }
// pub fn update_totp_key(mut storage: impl JulieStorage, client: ClientAuth)->Result<ClientAuth,S5ErrorKind>{
//     let mut updated = match storage.read(JulieDatabase::Client,&client.clone().uid).unwrap(){
//         JulieDatabaseItem::Client(client)=>client,
//         JulieDatabaseItem::Service(_)=>{ClientAuth::new()},
//     };

//     if updated.factors.contains(&AuthFactor::Totp){
//         return Err(S5ErrorKind::TotpKeyEstablished)
//     }
//     else{
//         updated.totp_key = keygen(Encoding::Base32);
//     }

//     if storage.update(JulieDatabaseItem::Client(updated.clone())).unwrap(){
//         Ok(updated.clone())
//     }
//     else{
//         Ok(client.clone())
//     }

// }
// pub fn update_email_status(mut storage: impl JulieStorage, client: ClientAuth)->Result<bool,S5ErrorKind>{
//     let mut updated = match storage.read(JulieDatabase::Client,&client.clone().uid).unwrap(){
//             JulieDatabaseItem::Client(client)=>client,
//             JulieDatabaseItem::Service(_)=>{panic!("OH NO! LOOK AT WHAT YOUVE DONE!")},
//     };

//     let expiry = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//         Ok(n) => n.as_secs() + EMAIL_TOKEN_LIFETIME,
//         Err(_) => panic!("SystemTime before UNIX EPOCH!"),
//     };

//     let token = aes::keygen(aes::Encoding::Hex);

//     updated.email_expiry = expiry;
//     updated.email_token = token;

//     if storage.update(JulieDatabaseItem::Client(updated.clone())).unwrap(){
//         Ok(true)
//     }
//     else{
//         Ok(false)
//     }
    

// }
// pub fn establish_totp(mut storage: impl JulieStorage, client: ClientAuth)->Result<ClientAuth,S5ErrorKind>{
//     let mut updated = match storage.read(JulieDatabase::Client,&client.clone().uid).unwrap(){
//         JulieDatabaseItem::Client(client)=>client,
//         JulieDatabaseItem::Service(_)=>{panic!("OH NO! LOOK AT WHAT YOUVE DONE!")},
//     };

//     if updated.factors.contains(&AuthFactor::Totp){
//         return Err(S5ErrorKind::TotpKeyEstablished)
//     }
//     else{
//         updated.factors.push(AuthFactor::Totp);
//     }

//     if storage.update(JulieDatabaseItem::Client(updated.clone())).unwrap(){
//         Ok(updated.clone())
//     }
//     else{
//         Ok(client.clone())
//     }

// }
/// Since this initializes the auth process, we return a ClientAuth, where other verification functions return bool
/// 
/// 
pub fn get_client(mut client_storage: Box<dyn JulieStorage>, uid: String)->Result<ClientAuth,S5ErrorKind>{
    match client_storage.read(JulieDatabase::Client,&uid).unwrap(){
        JulieDatabaseItem::Client(client)=>Ok(client),
        JulieDatabaseItem::Service(_)=>Err(S5ErrorKind::ServerError),
    }
}
pub fn get_service(mut service_storage: Box<dyn JulieStorage>, name: String)->Result<ServiceIdentity,S5ErrorKind>{
    match service_storage.read(JulieDatabase::Service,&name).unwrap(){
        JulieDatabaseItem::Client(_)=>Err(S5ErrorKind::ServerError),
        JulieDatabaseItem::Service(service)=>Ok(service),
    }
}

pub fn check_apikey(mut client_storage: Box<dyn JulieStorage>,uid: String, apikey: String) -> Result<bool,S5ErrorKind> {
    let client = match client_storage.read(JulieDatabase::Client,&uid){
        Ok(item)=> match item{
            JulieDatabaseItem::Client(client)=>client,
            JulieDatabaseItem::Service(_)=>{panic!("HOW CAN SHE SLAP?")},
        }
        Err(_)=>return Err(S5ErrorKind::NotInDatabase)
    };

    if client.verify_apikey(&uid, &apikey) {
        Ok(true)  
    }else{
        Err(S5ErrorKind::ApiKey)
    }
}
/// A warp filter for basic auth
pub fn check_basic_auth(
    mut client_storage: impl JulieStorage,
    uid: String,
    basic_auth_encoded: String,
) -> Result<bool,S5ErrorKind> {
    let client = match client_storage.read(JulieDatabase::Client,&uid).unwrap(){
        JulieDatabaseItem::Client(client)=>client,
        JulieDatabaseItem::Service(_)=>{panic!("HOW CAN SHE SLAP?")},
    };

    let trimmed = basic_auth_encoded.replace("Basic ", "");
    let status = client.verify_basic_auth(trimmed);

    if status {
        Ok(true)
    } else {
        return Err(S5ErrorKind::BasicAuth);
    }
}
/// A warp filter for signature auth
// pub fn check_signature(
//     mut client_storage: impl JulieStorage,
//     uid: String,
//     signature: String,
//     timestamp: u64,
// ) -> Result<bool,S5ErrorKind> {
//     let client = match client_storage.read(JulieDatabase::Client,&uid).unwrap(){
//         JulieDatabaseItem::Client(client)=>client,
//         JulieDatabaseItem::Service(_)=>{panic!("HOW CAN SHE SLAP?")},
//     };

//     let message = "timestamp=".to_string() + &timestamp.to_string();
//     let mut verify = true;
//     if client.factors.contains(&AuthFactor::Signature) || client.factors.contains(&AuthFactor::All) {
//         verify = client.verify_signature( &message, &signature);
//     }

//     if verify {
//         Ok(true)
//     } else {
//         return Err(S5ErrorKind::Signature);
//     }
// }
// /// A warp filter for totp auth



#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::hash::sha256;
    use crate::lib::rsa;
    use crate::storage::sled::{SledDb, init as init_sled};
    use crate::lib::jwt;
    use oath::{HashType};
    use crate::lib::totp;

    #[test]
    fn core_composite() { 
        let client_auth = ClientAuth::new();
        // let config = SledConfig{
        //     db: "client".to_string()
        // };
        let mut client_storage = Box::new(init_sled(JulieDatabase::Client).unwrap());
        assert!(client_storage.create(JulieDatabaseItem::Client(client_auth.clone())).unwrap());

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

        assert!(update_basic_auth(client_storage,client_auth.clone().uid, username, &p256.clone()));

        let public_key = "-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAqkVu2BX3K2ZB+0F+dGor\necTfBM9GYqNxxn3tTMR61fEMBX0vPA5itSQcfh91pofKrvC65CZBnu71EElvo4hU\n9WTqjiaNJJDB3dxLbek2WEx57kCM7vewiwyosUdeBeqdxZX/Tp1xHEyB636J/L4R\nGan7XDGfWs47ZnnmR/CB13LuaHW08ej9WWNiy8UPs0LRzUZkwDNdnhec/N+j5GG0\nTBqwcgfaQDep4irtCuCQ9Q1pXrzgFEwqc0Qsr/F7V2cdJLvtLhG9CW6RZZdlNYbc\nIVNi+G7kVlSts7/81/EsjSAL8VMcvvj6CakBFzyUH4kgQRvlwVA3grL/7d39Wu5F\nBFPVm/40nSMnh28J0Sk/2E5Xt7xSQ9A43WM9mUNLSXkuEZbvMY09yzxzUZo9paPG\nbvKJY72tdmNvc2La0gaEhGlQf+7IDOs9uUBkOw0f+wyzM9bLNiQqLpeQ7cQH9rIT\nV4I+tbo4jEmI5vZwB2AImbsVXEn8z9OxV4TBqBciwi0jECcu5yh6b2cS/Gj7D+5x\nEGvtKO26/Iqpfrzf1Of7unF8DdYz8hZdGZ3Vs3di0apksmwbw7soNk6Q2R/c+c0X\nXneQKZxmDkvOPna1Zldx9n0WSloq+neDdwt0D9DyPORSad1/o1+grg6ksTylX72b\njO+9ZXTV/bfznGJI2ZojOGsCAwEAAQ==\n-----END PUBLIC KEY-----";
        let private_key = "-----BEGIN RSA PRIVATE KEY-----\n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\n-----END RSA PRIVATE KEY-----";
        
        // assert!(update_public_key(client_storage,client_auth.clone().uid, public_key));
        // let message = "timestamp=now";
        // let updated = get_client(client_storage,client_auth.uid).unwrap();
        // let signature = rsa::sign(message, private_key);

        // assert!(updated.verify_apikey(&updated.clone().uid, &updated.clone().apikey));
        // println!("{:?},{}",updated.clone(),encoded.clone());
        // assert!(updated.verify_basic_auth(encoded));
        // assert!(updated.verify_signature( message, &signature));

        // let service_name = "satoshipay";
        // let shared_secret = keygen(Encoding::Hex);

        // let service = ServiceIdentity::new(service_name,&shared_secret);
        // let mut service_storage = init_sled(JulieDatabase::Service).unwrap();
        // assert!(service_storage.create(JulieDatabaseItem::Service(service.clone())).unwrap());

        // let token = service.issue_token(updated.clone().uid).unwrap();
        // println!("Bearer {:#?}",token.clone());

        // let verify = jwt::verify(token,service.clone().shared_secret).unwrap();
        // println!("{:#?}",verify);

        // // Upgrade client to mfa
        // let mfa_client = update_totp_key(client_storage,updated.clone()).unwrap();

        // let otp = totp::generate_otp(mfa_client.clone().totp_key, HashType::SHA1);
        // assert!(mfa_client.verify_totp(otp));

        // let mfa_client = match client_storage.read(JulieDatabase::Client,&mfa_client.uid).unwrap(){
        //         JulieDatabaseItem::Client(client)=>client,
        //         JulieDatabaseItem::Service(_)=>panic!("OH NO! LOOK AT WHAT YOUVE DONE!"),
        
        // };
        // let token = service.issue_token(mfa_client.clone().uid).unwrap();
        // println!("Bearer {:#?}",token.clone());

        // let verify = jwt::verify(token,service.clone().shared_secret).unwrap();
        // println!("{:#?}",verify);

        // println!("{:#?}",mfa_client.clone());

        // // Comment out the following if you want a user to persist for bash testing
        // assert!(client_storage.delete(JulieDatabase::Client,&mfa_client.uid).unwrap());
        // assert!(service_storage.delete(JulieDatabase::Service,&service.clone().name).unwrap());
        // ()


    }

    #[test] #[ignore]
    fn core_email_composite(){
        // let config = SledConfig{
        //     db: "client".to_string()
        // };
        let storage = init_sled(JulieDatabase::Client).unwrap();
        let client_auth = ClientAuth::new();
        // admin gives client this new client_auth with an apikey
        // client then registers a username and password
        let email = "vishalmenon.92@gmail.com";
        // let _client_auth = update_email(storage,client_auth.clone().uid, email);
        // client_auth.send_email_token();
        // assert!(storage.delete(&client_auth.uid).unwrap())
    }
}