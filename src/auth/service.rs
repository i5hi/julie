/// NOTE: 
/// Every time you create a tree, you have to call drop on it if it is not used or else it leaves behind an empty index.

use crate::lib::jwt;

use serde::{Deserialize, Serialize};
use std::str;

use uuid::Uuid;

/// ServiceOdentity is a database structure to store service id and shared_secret data to sign authenticated tickets for specific services.
/// The current implementation is very tightly coupled with sled db.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceIdentity {
    pub sid: String, // index
    pub name: String,
    pub shared_secret: String,
}

impl ServiceIdentity {
    /// Used by the admin to create a new client with a sid and apikey index.
    pub fn new(name: &str, shared_secret: &str) -> Self {

        ServiceIdentity {
            sid: format!("s5sid-{}", Uuid::new_v4()).to_string(),
            name: name.to_string(),
            shared_secret: shared_secret.to_string(),
        }
   
    }
    
    pub fn dummy() -> Self {

        ServiceIdentity {
            sid: format!("s5sid-{}", Uuid::new_v4()).to_string(),
            name: format!("s5sid-{}", Uuid::new_v4()).to_string(),
            shared_secret: format!("s5sid-{}", Uuid::new_v4()).to_string(),
        }
   
    }

    pub fn issue_token(&self, uid: String)->Option<String>{
        let token = jwt::issue(uid.to_string(), self.shared_secret.to_string(), self.name.to_string(), "Will be a comma separated list of auth methods.".to_string());
        Some(token)
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::aes;

    #[test]
    fn service_composite() {
        // client asks admin to initialize a user account
        let shared_secret = aes::keygen(aes::Encoding::Hex);

        let service_id = ServiceIdentity::new("satoshipay",&shared_secret);
        // admin gives client this new client_auth with an apikey

    }

}
