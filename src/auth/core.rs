use std::str;

use crate::auth::client::{ClientAuth,AuthLevel};
use crate::auth::service::{ServiceIdentity};

use crate::lib::hash;
use crate::lib::rsa;
use crate::lib::totp;
use crate::lib::jwt;

use crate::lib::error::S5ErrorKind;

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
    use crate::lib::rsa;
    macro_rules! wait {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }
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

        let registered_client = wait!(client_auth.update_basic_auth(username, &p256));
        let registered_client = registered_client.update_level(AuthLevel::Basic);
   
        let ready_client = verify_apikey(&registered_client.apikey).unwrap();
        let basic_status = verify_basic_auth(ready_client.clone(), encoded);
        assert!(basic_status);
      
        let service_name = "satoshipay";
        let service = ServiceIdentity::new(service_name);

        let token = issue_token(ready_client.clone(), service_name).unwrap();
        println!("Bearer {:#?}",token.clone());

        let verify = jwt::verify(token,service.shared_secret).unwrap();
        println!("{:#?}",verify);


    }
}