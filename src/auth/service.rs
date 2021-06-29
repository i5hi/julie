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
    /// Get ServiceIdentity structure using name
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
        let root = database::get_root(database::SERVICE).unwrap();
        let main_tree = database::get_tree(root.clone(), sid).unwrap();

        // if this tree exists return it
        if main_tree.contains_key(b"name").unwrap() {
            root.flush().unwrap();
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
            root.drop_tree(&main_tree.name()).unwrap();
            root.flush().unwrap();
            None
        }
    }
    pub fn update_shared_secert(&self, shared_secret: &str) -> Self {
        let root = database::get_root(database::SERVICE).unwrap();
        let main_tree = database::get_tree(root, &self.clone().sid).unwrap();
    
        main_tree.insert(b"shared_secret", shared_secret.as_bytes()).unwrap();
        main_tree.flush().unwrap();
        let mut updated = self.clone();
        updated.shared_secret = shared_secret.to_string();
        updated.clone()
    
    }
 
 
    pub fn delete(&self)->bool{
    
        let root = database::get_root(database::SERVICE).unwrap();
        // println!("{:?}", str::from_utf8(&main_tree.name()).unwrap());
        // println!("{:?}", str::from_utf8(&apikey_tree.name()).unwrap());
        let main_tree = database::get_tree(root.clone(), &self.clone().sid).unwrap();
        let name_tree = database::get_tree(root.clone(), &self.clone().name).unwrap();

        main_tree.clear().unwrap();
        main_tree.flush().unwrap();
        root.drop_tree(&main_tree.name()).unwrap();
        name_tree.clear().unwrap();
        name_tree.flush().unwrap();
        root.drop_tree(&name_tree.name()).unwrap();

        root.flush().unwrap();

        true

    }

}

/// All methods use sid as the primary index. Incase only an name is presented, the sod index can be retrieved with this function.
fn get_sid_from(name: &str) -> Option<String> {
    let root = database::get_root(database::SERVICE).unwrap();
    let name_tree = database::get_tree(root.clone(), name).unwrap();

    if name_tree.contains_key(b"sid").unwrap() {
       Some(str::from_utf8(&name_tree.get(b"sid").unwrap().unwrap().to_vec()).unwrap().to_string())
    } else {
        root.drop_tree(&name_tree.name()).unwrap();
        None
    }

}

/// Retrives all tree indexes in a db
pub fn _get_sid_indexes() -> Vec<String>{
    let root = database::get_root(database::SERVICE).unwrap();
    let mut uids: Vec<String> = [].to_vec();
    for key in root.tree_names().iter() {
        let uid = str::from_utf8(key).unwrap();
        if uid.starts_with("s5sid"){
            uids.push(uid.to_string());
        }
        else{

        };
    }
    uids
}
/// Retrives all tree indexes in a db
pub fn get_name_indexes() -> Vec<String>{
    let root = database::get_root(database::SERVICE).unwrap();
    let mut names: Vec<String> = [].to_vec();
    for key in root.tree_names().iter() {
        let name = str::from_utf8(key).unwrap();
        if !name.starts_with("s5sid") && name != "__sled__default"{
            names.push(name.to_string());
        }
        else{

        };
    }
    names
}
/// Removes all trees in a db. Careful with that axe, Eugene.
pub fn _remove_service_trees() -> bool {
    let root = database::get_root(database::SERVICE).unwrap();
    for key in root.tree_names().iter() {
        let index = str::from_utf8(key).unwrap();
        let tree = database::get_tree(root.clone(),index).unwrap();
        // println!("Name: {:?}",str::from_utf8(&tree.name()).unwrap());
        tree.clear().unwrap();
        tree.flush().unwrap();
        if str::from_utf8(&tree.name()).unwrap() != "__sled__default" {
            root.drop_tree(&tree.name()).unwrap();
        }
        else{

        }

    }
    root.flush().unwrap();

    true
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
