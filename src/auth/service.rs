use crate::lib::aes;
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
    pub fn new(name: &str) -> Self {
        let root = database::get_root(database::SERVER_ID_TREE).unwrap();
        let sid = format!("s5sid-{}", Uuid::new_v4());
        let main_tree = database::get_tree(root.clone(), &sid).unwrap();

        let name_tree = database::get_tree(root.clone(), &name.clone()).unwrap();

        // creating an alternative apikey index tree
        name_tree.insert(b"sid", sid.as_bytes()).unwrap();
        name_tree.insert(b"name", name.as_bytes()).unwrap();

        let shared_secret = aes::keygen(aes::Encoding::Hex);

        // creating main tree
        main_tree.insert(b"sid", sid.as_bytes()).unwrap();
        main_tree.insert(b"name", name.as_bytes()).unwrap();
        main_tree.insert(b"shared_secret", shared_secret.as_bytes()).unwrap();

        main_tree.flush().unwrap();

        ServiceIdentity {
            sid: sid.to_string(),
            name: name.to_string(),
            shared_secret: shared_secret.to_string(),
        }
    }
    /// Get ServiceIdentity structure using apikey
    pub fn init(name: &str) -> Option<Self>{
        let sid = match get_sid_from(name){
            Some(sid)=>sid,
            None=> return None
        };

        match ServiceIdentity::read(&sid){
            Some(object)=> return Some(object),
            None=> return None
        };

    }
    /// Get a ServiceIdentity structure using uid
    pub fn read(sid: &str) -> Option<Self> {
        let root = database::get_root(database::SERVER_ID_TREE).unwrap();
        let main_tree = database::get_tree(root, sid).unwrap();

        // if this tree exists return it
        if main_tree.contains_key(b"name").unwrap() {
            Some(ServiceIdentity {
                sid: str::from_utf8(&main_tree.get(b"sid").unwrap().unwrap().to_vec())
                    .unwrap()
                    .to_string(),
                name: str::from_utf8(&main_tree.get(b"name").unwrap().unwrap().to_vec())
                    .unwrap()
                    .to_string(),
                shared_secret: str::from_utf8(&main_tree.get(b"shared_secret").unwrap().unwrap().to_vec())
                    .unwrap()
                    .to_string(),
            })
        } else {
            None
        }
    }
    pub async fn update_shared_secert(&self, shared_secret: &str) -> Self {
        let root = database::get_root(database::SERVER_ID_TREE).unwrap();
        let main_tree = database::get_tree(root, &self.clone().sid).unwrap();
    
        main_tree.insert(b"shared_secret", shared_secret.as_bytes()).unwrap();
        main_tree.flush().unwrap();
        let mut updated = self.clone();
        updated.shared_secret = shared_secret.to_string();
        updated.clone()
    
    }
 
 
    pub fn delete(&self)->bool{
        let root = database::get_root(database::SERVER_ID_TREE).unwrap();
        let main_tree = database::get_tree(root.clone(), &self.sid).unwrap();
        let name_tree = database::get_tree(root.clone(), &self.name.clone()).unwrap();

        name_tree.remove(b"sid").unwrap();
        name_tree.remove(b"name").unwrap();
        
        main_tree.remove(b"sid").unwrap();
        main_tree.remove(b"name").unwrap();
        main_tree.remove(b"shared_secret").unwrap();

        true

    }

}

/// All methods use sid as the primary index. Incase only an name is presented, the sod index can be retrieved with this function.
fn get_sid_from(name: &str) -> Option<String> {
    let root = database::get_root(database::SERVER_ID_TREE).unwrap();
    let name_tree = database::get_tree(root.clone(), name).unwrap();

    if name_tree.contains_key(b"sid").unwrap() {
       Some(str::from_utf8(&name_tree.get(b"sid").unwrap().unwrap().to_vec()).unwrap().to_string())
    } else {
        None
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticket_storage_composite() {
        // client asks admin to initialize a user account
        let service_id = ServiceIdentity::new("satoshipay");
        // admin gives client this new client_auth with an apikey
        assert_eq!(service_id.clone().delete(),true);
        println!("{:#?}",service_id);
        let delete_status = match ServiceIdentity::read(&service_id.sid){
            Some(_)=>false,
            None=>true
        };

        assert!(delete_status);
    }


}
