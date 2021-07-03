/// NOTE: 
/// Every time you create a tree, you have to call drop on it if it is not used or else it leaves behind an empty index.

use crate::lib::database;

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
   
        let root = database::get_root(database::SERVICE).unwrap();
        let sid = format!("s5sid-{}", Uuid::new_v4());
        let main_tree = database::get_tree(root.clone(), &sid.clone()).unwrap();
        let name_tree = database::get_tree(root.clone(), &name.clone()).unwrap();

        // creating an alternative name index tree
        println!("HERE");
        name_tree.insert(b"sid", sid.clone().as_bytes()).unwrap();
        name_tree.insert(b"name", name.as_bytes()).unwrap();


        // creating main tree
        main_tree.insert(b"sid", sid.clone().as_bytes()).unwrap();
        main_tree.insert(b"name", name.as_bytes()).unwrap();
        main_tree.insert(b"shared_secret", shared_secret.as_bytes()).unwrap();
        
        name_tree.flush().unwrap();
        main_tree.flush().unwrap();
        root.flush().unwrap();

        ServiceIdentity {
            sid: sid.to_string(),
            name: name.to_string(),
            shared_secret: shared_secret.to_string(),
        }
   
    }


    pub fn issue_token(&self, service: ServiceIdentity)->Option<String>{
        let token = jwt::issue(self.uid.to_string(), service.shared_secret, service.name, "Will be a comma separated list of auth methods.".to_string());
        Some(token)
    }

}
// WARNING BREKAING VERSION
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
        let indexes = _get_sid_indexes();
        println!("{:#?}",_get_sid_indexes());
        println!("{:#?}",get_name_indexes());

        
        assert!(indexes.contains(&service_id.clone().sid));
        assert_eq!(service_id.clone().delete(),true);

        let delete_status = match ServiceIdentity::read(&service_id.sid){
            Some(_)=>false,
            None=>true
        };

        assert!(delete_status);
    }

        // Careful with that axe, Eugene
        /// This must always be ignored on master or it will delete all your stuff
        #[test] #[ignore]
        fn delete_all_services(){
            let status = _remove_service_trees();
            assert!(status);
            assert_eq!(_get_sid_indexes().len(),0);
    
        }


}
