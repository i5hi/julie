use std::fmt::Display;

use crate::lib::aes;
use crate::lib::database;
use crate::lib::hash;

use serde::{Deserialize, Serialize};
use std::str;

use std::str::FromStr;

use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthLevel{
    /// ApiKey is level 0, when a random new uid and apikey are generated by an admin and shared with a client to setup Basic auth.
    ApiKey,
    /// Basic signifies a username:pass256 set by the client (requires ApiKey).
    Basic,
    /// Signature signifies a public_key is registered; requiring token requests to be signed by the client (requires Basic). 
    Signature,
    /// Totp is if a b32_key is verified by the client (requires Basic).
    Totp,
    /// MultiFactor is if both Totp and Signature are required.
    MultiFactor,
}
impl AuthLevel {
    pub fn as_str(&self) -> &'static str {
        match *self {
            AuthLevel::ApiKey=>"ApiKey",
            AuthLevel::Basic=>"Basic",
            AuthLevel::Signature=>"Signature",
            AuthLevel::Totp=>"Totp",
            AuthLevel::MultiFactor=>"MultiFactor"
        }
    }
}
impl FromStr for AuthLevel{
    type Err = ();

    fn from_str(s: &str) -> Result<AuthLevel,Self::Err>{
        let level = match s {
            "ApiKey"=>AuthLevel::ApiKey,
            "Basic"=>AuthLevel::Basic,
            "Signature"=>AuthLevel::Signature,
            "Totp"=>AuthLevel::Totp,
            "MultiFactor"=>AuthLevel::MultiFactor,
            &_=>return Err(())
        };
        Ok(level)
    }
}
/// ClientAuth is a database structure to store client authentication data.
//// The current implementation is very tightly coupled with sled db.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientAuth {
    pub uid: String, // index
    pub apikey: String,
    pub username: String,
    pub pass256: String,
    pub public_key: String,
    pub totp_key: String,
    pub level: AuthLevel,

}

impl ClientAuth {
    /// Used by the admin to create a new client with a uid and apikey index.
    pub fn new() -> Self {
        let root = database::get_root(database::CLIENT_AUTH_TREE).unwrap();
        let uid = format!("s5uid-{}", Uuid::new_v4());
        let main_tree = database::get_tree(root.clone(), &uid).unwrap();

        let apikey = aes::keygen(aes::Encoding::Hex);
        let apikey_tree = database::get_tree(root.clone(), &apikey.clone()).unwrap();

        // creating an alternative apikey index tree
        apikey_tree.insert(b"uid", uid.as_bytes()).unwrap();
        apikey_tree.insert(b"api_key", uid.as_bytes()).unwrap();

        // creating main tree
        main_tree.insert(b"uid", uid.as_bytes()).unwrap();
        main_tree.insert(b"apikey", apikey.as_bytes()).unwrap();
        main_tree.insert(b"username", "none".as_bytes()).unwrap();
        main_tree.insert(b"pass256", "none".as_bytes()).unwrap();
        main_tree.insert(b"public_key", "none".as_bytes()).unwrap();
        main_tree.insert(b"totp_key", "none".as_bytes()).unwrap();
        main_tree.insert(b"level", format!("{:#?}",AuthLevel::ApiKey).as_bytes()).unwrap();

        main_tree.flush().unwrap();

        ClientAuth {
            uid: uid.to_string(),
            apikey: apikey.to_string(),
            username: "none".to_string(),
            pass256: "none".to_string(),
            public_key: "none".to_string(),
            totp_key: "none".to_string(),
            level:AuthLevel::ApiKey
        }
    }
    /// Get ClientAuth structure using apikey
    pub fn init(apikey: &str) -> Option<Self>{
        let uid = match get_uid_from(apikey){
            Some(uid)=>uid,
            None=> return None
        };

        match ClientAuth::read(&uid){
            Some(object)=> return Some(object),
            None=> return None
        };

    }
    /// Get a ClientAuth structure using uid
    pub fn read(uid: &str) -> Option<Self> {
        let root = database::get_root(database::CLIENT_AUTH_TREE).unwrap();
        let main_tree = database::get_tree(root, uid).unwrap();

        // if this tree exists return it
        if main_tree.contains_key(b"apikey").unwrap() {
            Some(ClientAuth {
                uid: str::from_utf8(&main_tree.get(b"uid").unwrap().unwrap().to_vec())
                    .unwrap()
                    .to_string(),
                apikey: str::from_utf8(&main_tree.get(b"apikey").unwrap().unwrap().to_vec())
                    .unwrap()
                    .to_string(),
                username: str::from_utf8(&main_tree.get(b"username").unwrap().unwrap().to_vec())
                    .unwrap()
                    .to_string(),
                pass256: str::from_utf8(&main_tree.get(b"pass256").unwrap().unwrap().to_vec())
                    .unwrap()
                    .to_string(),
                public_key: str::from_utf8(&main_tree.get(b"public_key").unwrap().unwrap().to_vec())
                    .unwrap()
                    .to_string(),
                totp_key:  str::from_utf8(&main_tree.get(b"totp_key").unwrap().unwrap().to_vec())
                .unwrap()
                .to_string(),
                level: AuthLevel::from_str(str::from_utf8(&main_tree.get(b"level").unwrap().unwrap().to_vec()).unwrap()).unwrap(),
            })
        } else {
            None
        }
    }
    pub async fn update_basic_auth(&self, username: &str, pass256: &str) -> Self {
        let root = database::get_root(database::CLIENT_AUTH_TREE).unwrap();
        let main_tree = database::get_tree(root, &self.clone().uid).unwrap();
    
        main_tree.insert(b"username", username.as_bytes()).unwrap();
        main_tree.insert(b"pass256", hash::sha256(pass256).as_bytes()).unwrap();
        main_tree.flush().unwrap();
        let mut updated = self.clone();
        updated.username = username.to_string();
        updated.pass256 = hash::sha256(pass256);
        updated.clone()
    
    }
    pub fn update_public_key(&self, public_key: &str) -> Self {
        let root = database::get_root(database::CLIENT_AUTH_TREE).unwrap();
        let main_tree = database::get_tree(root, &self.clone().uid).unwrap();

        main_tree.insert(b"public_key", public_key.as_bytes()).unwrap();
        main_tree.flush().unwrap();
        let mut updated = self.clone();
        updated.public_key = public_key.to_string();
        updated.clone()
    }
    pub async fn update_totp_key(&self, key: &str) -> Self {
        let root = database::get_root(database::CLIENT_AUTH_TREE).unwrap();
        let main_tree = database::get_tree(root, &self.clone().uid).unwrap();
    
        main_tree.insert(b"totp_key", key.as_bytes()).unwrap();
        main_tree.flush().unwrap();

        let mut updated = self.clone();
        updated.totp_key = key.to_string();
        updated.clone()

    
    }
    /// It is up to the implementor to appropriately update a clients AuthLevel
    pub fn update_level(&self,level:AuthLevel)->Self{
        let root = database::get_root(database::CLIENT_AUTH_TREE).unwrap();
        let main_tree = database::get_tree(root, &self.clone().uid).unwrap();
        main_tree.insert(b"level", format!("{:#?}",level).as_bytes()).unwrap();
        main_tree.flush().unwrap();
        let mut updated = self.clone();
        updated.level = level;
        updated.clone()
    }
    pub fn delete(&self)->bool{
        let root = database::get_root(database::CLIENT_AUTH_TREE).unwrap();
        let main_tree = database::get_tree(root.clone(), &self.uid).unwrap();
        let apikey_tree = database::get_tree(root.clone(), &self.apikey.clone()).unwrap();

        apikey_tree.remove(b"uid").unwrap();
        apikey_tree.remove(b"apikey").unwrap();
        
        main_tree.remove(b"uid").unwrap();
        main_tree.remove(b"apikey").unwrap();
        main_tree.remove(b"username").unwrap();
        main_tree.remove(b"pass256").unwrap();
        main_tree.remove(b"public_key").unwrap();
        main_tree.remove(b"totp_key").unwrap();
        main_tree.remove(b"level").unwrap();


        true

    }

}

/// All methods use uid as the primary index. Incase only an apikey is presented, the uid index can be retrieved with this function.
fn get_uid_from(apikey: &str) -> Option<String> {
    let root = database::get_root(database::CLIENT_AUTH_TREE).unwrap();
    let apikey_tree = database::get_tree(root.clone(), apikey).unwrap();

    if apikey_tree.contains_key(b"uid").unwrap() {
       Some(str::from_utf8(&apikey_tree.get(b"uid").unwrap().unwrap().to_vec()).unwrap().to_string())
    } else {
        None
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::hash::sha256;
    use std::fs::File;
    use std::io::prelude::*;

    macro_rules! wait {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn auth_composite() {
        // client asks admin to initialize a user account
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
        let public_key = "-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAuvzpR/gruC+W/JAy7amw\nchCOaM7U/pUuMLy6JcE+Y8GTtbVqUi8MX+JeJOdEa/H6o2v99lJtUfYFdpU5cman\nfn38h7bDSw+EsqPFgmO4RrASTHiPJ+s8FU/3SbV5tguSBTOEmbiTc5x0IAAmlrLs\nAwUHEypz9ug+OIWQt0YAoYBfApTq8rV+TaYe5NxL2hbtFKZemcIGxfn3mgn6B2Rs\nZeOOnCB661MXBYPJl2+j2HwbF3pWHZZUCXKB7t5krPJScAlEFAZsDCR4Gkzu0tF/\nm+F7cId3sTBGX2Ci1FrqctfXbfzLv2BTIbKg+4YyCgX3Hr+XfqI4tEuGK7wb3zMg\nBmr7d6Kuwf5VHDIBifu31vZ6w2Z6JzUFpeL7FJGeFjEZ4xk+mvVdG9uC3W9vYrcR\nHZ1CMllMGDs+8Y6BVdYFgFwYt/ht53vij4psSXIewdiBignUSiuC5BGRUpEtNhJq\niKDsHZmjtCwsscP+XhaBwALLI7JFvdq8ELMP4SwxFILGbWmArs9+lOfavnux3zf/\nyWKt5OcKmZL/Ns2o46+Q5PIIMU53XyMSuDXz70QKib9yNRswJj/lMX/+j1JiprHw\nMW3UiFMz45QJ7FFAGsN542GNXQhKQ9Z86rwUT04GQ5ArlUO1PnhIWFZaYrCoogYS\n1tpQMyInFq8zBypTJnh5iTUCAwEAAQ==\n-----END PUBLIC KEY-----";
        let configured_client = registered_client.update_public_key(&public_key);
        let configured_client = configured_client.update_level(AuthLevel::Signature);

        println!("{:#?}", configured_client);

        let read_client = ClientAuth::read(&configured_client.uid).unwrap();
        println!("{:#?}", read_client.clone());

        assert_eq!(
            get_uid_from(&read_client.clone().apikey).unwrap(),
            read_client.clone().uid
        );
        assert_eq!(read_client.clone().delete(),true);

        let delete_status = match ClientAuth::read(&read_client.uid){
            Some(_)=>false,
            None=>true
        };

        assert!(delete_status);
    }


    #[test]
    fn init_bash_test() {
        // uncomment delete file to persist this user
        // make sure you delete it afterwards. 
        // rerunning the test will override the old apikey.txt 
        // this will make iit a pain to dig out that apikey index from sled
        // its okay though, will be a good exercise
        let client_auth = ClientAuth::new();
        assert!(client_auth.clone().delete());

        println!("{:#?}", client_auth);
        let mut _file = File::create("apikey").unwrap();
        _file = File::open("apikey").unwrap();

        let mut contents = client_auth.clone().apikey;
        _file.read_to_string(&mut contents).unwrap();


    }
}
