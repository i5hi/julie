
use sled::{Db, Tree};
use crate::storage::interface::{JulieStorage, JulieDatabase};
use crate::auth::client::{ClientAuth,AuthFactor};
use crate::auth::service::{ServiceIdentity};

use std::env;
// use serde::{Deserialize, Serialize};
// use std::str::FromStr;
use std::str;

pub const STORAGE_ROOT: &str = ".julie"; // Database

#[derive(Debug)]
pub struct SledConfig{
    db: String,
}

pub type SledDb = Db;

impl JulieStorage for SledDb {
    fn init(db: JulieDatabase) -> Result<Self, String>{
        match get_root(db){
            Ok(root)=>{
                Ok(root)
            },
            Err(e)=>{
                Err(e)
            }
        }
    }
    fn create(&mut self, db: JulieDatabase, object: (ClientAuth,ServiceIdentity)) -> Result<bool, String> {
        match  db{
            JulieDatabase::Client=>{
                let main_tree = get_tree(self.clone(), &object.0.uid).unwrap();
                let bytes = bincode::serialize(&object).unwrap();
                main_tree.insert("client", bytes).unwrap();
                Ok(true)

            }
            JulieDatabase::Service=>{
                let main_tree = get_tree(self.clone(), &object.1.name).unwrap();
                let bytes = bincode::serialize(&object).unwrap();
                main_tree.insert("service", bytes).unwrap();
                Ok(true)
            }
        }
      
    }
    fn read(&mut self,db: JulieDatabase, index: &str)-> Result<(ClientAuth,ServiceIdentity),String>{
        match db {
            JulieDatabase::Client=>{
                match get_tree(self.clone(), index){
                    Ok(tree)=>{
                         // if this tree exists return it
                         if tree.contains_key(b"client").unwrap() {
                             match tree.get("client").unwrap() {
                                 Some(bytes) => {
                                     let client: ClientAuth = bincode::deserialize(&bytes).unwrap();
                                     Ok((client,ServiceIdentity::dummy()))
                                 },
                                 None => Err("No client found in database".to_string()),
                             }
                         } else {
                             self.drop_tree(&tree.name()).unwrap();
                             Err("No client found in database".to_string())
                         }
                    }
                    Err(_)=>{
                        Err("Could not read the client stuffs".to_string())
                    }
                }

            }
            JulieDatabase::Service=>{
                match get_tree(self.clone(), index){
                    Ok(tree)=>{
                         // if this tree exists return it
                         if tree.contains_key(b"service").unwrap() {
                             match tree.get("service").unwrap() {
                                 Some(bytes) => {
                                    let service: ServiceIdentity = bincode::deserialize(&bytes).unwrap();
                                    Ok((ClientAuth::new(),service))
                                 },
                                 None => Err("No service found in database".to_string()),
                             }
                         } else {
                             self.drop_tree(&tree.name()).unwrap();
                             Err("No service found in database".to_string())
                         }
                    }
                    Err(_)=>{
                        Err("Could not read the service stuffs".to_string())
                    }
                }
            }
        }
     
    }
    fn update(&mut self,db: JulieDatabase, object: (ClientAuth,ServiceIdentity)) -> Result<bool, String>{
        match db {
            JulieDatabase::Client=>{
          
                let main_tree = get_tree(self.clone(), &object.0.clone().uid).unwrap();
        
                let bytes = bincode::serialize(&object.0).unwrap();
        
                main_tree.insert("client", bytes).unwrap();
                main_tree.flush().unwrap();
                Ok(true)
            }
            JulieDatabase::Service=>{
                let main_tree = get_tree(self.clone(), &object.1.clone().name).unwrap();
        
                let bytes = bincode::serialize(&object.1).unwrap();
        
                main_tree.insert("client", bytes).unwrap();
                main_tree.flush().unwrap();
                Ok(true)
            }
        }
     

    }
    fn delete(&mut self, index: &str)-> Result<bool,String>{
        let tree = get_tree(self.clone(), index).unwrap();
        tree.clear().unwrap();
        tree.flush().unwrap();
        self.drop_tree(&tree.name()).unwrap();
    
        Ok(true)

    }
}


/// Retrieves the primary data store @ $HOME/.julie/$db.
fn get_root(db: JulieDatabase) -> Result<Db, String> {
    let db_storage_path: String =
        format!("{}/{}/{}", env::var("HOME").unwrap(), STORAGE_ROOT, &db.to_string()).to_string();
    match sled::open(db_storage_path.clone()) {
        Ok(db) => Ok(db),
        Err(e) =>{
            println!("{:#?}",e);
            Err(format!("E:DB Open @ {} FAILED.", db_storage_path).to_string())
        }
    }
}

/// Retrieves a specific tree from the selected db by its index.
/// Client index is uid.
/// Service index is name.
fn get_tree(root: Db, index: &str) -> Result<Tree, String> {
    match root.open_tree(index.clone().as_bytes()) {
        Ok(tree) => Ok(tree),
        Err(_) => Err(format!("E:Tree Open @ {} FAILED.", index).to_string()),
    }
}

/// All methods use uid as the primary index. Incase only an apikey is presented, the uid index can be retrieved with this function.
fn get_uid_from(apikey: &str) -> Option<String> {
    let root = get_root(JulieDatabase::Client).unwrap();
    let apikey_tree = get_tree(root.clone(), apikey).unwrap();

    if apikey_tree.contains_key(b"uid").unwrap() {
       Some(str::from_utf8(&apikey_tree.get(b"uid").unwrap().unwrap().to_vec()).unwrap().to_string())
    } else {
        root.drop_tree(&apikey_tree.name()).unwrap();
        None
    }

}

/// Retrives all tree indexes in a db
pub fn get_uid_indexes() -> Vec<String>{
    let root = get_root(JulieDatabase::Client).unwrap();
    let mut uids: Vec<String> = [].to_vec();
    for key in root.tree_names().iter() {
        let uid = str::from_utf8(key).unwrap();
        if uid.starts_with("s5uid"){
            uids.push(uid.to_string());
        }
        else{

        };
    }
    uids
}
/// Removes all trees in a db. Careful with that axe, Eugene.
pub fn remove_client_trees() -> bool {
    let root = get_root(JulieDatabase::Client).unwrap();
    for key in root.tree_names().iter() {
        let index = str::from_utf8(key).unwrap();
        let tree = get_tree(root.clone(),index).unwrap();
        // println!("Name: {:?}",str::from_utf8(&tree.name()).unwrap());
        tree.clear().unwrap();
        tree.flush().unwrap();
        if str::from_utf8(&tree.name()).unwrap() != "__sled__default" {
            root.drop_tree(&tree.name()).unwrap();
        }
        else{

        }

    }
    // root.flush().unwrap();

    true
}



/// All methods use name as the primary index. Incase only an name is presented, the sod index can be retrieved with this function.
fn get_name_from(name: &str) -> Option<String> {
    let root = get_root(JulieDatabase::Service).unwrap();
    let name_tree = get_tree(root.clone(), name).unwrap();

    if name_tree.contains_key(b"name").unwrap() {
       Some(str::from_utf8(&name_tree.get(b"name").unwrap().unwrap().to_vec()).unwrap().to_string())
    } else {
        root.drop_tree(&name_tree.name()).unwrap();
        None
    }

}

/// Retrives all tree indexes in a db
pub fn _get_name_indexes() -> Vec<String>{
    let root = get_root(JulieDatabase::Service).unwrap();
    let mut uids: Vec<String> = [].to_vec();
    for key in root.tree_names().iter() {
        let uid = str::from_utf8(key).unwrap();
        if uid.starts_with("s5name"){
            uids.push(uid.to_string());
        }
        else{

        };
    }
    uids
}
/// Retrives all tree indexes in a db
pub fn get_name_indexes() -> Vec<String>{
    let root = get_root(JulieDatabase::Service).unwrap();
    let mut names: Vec<String> = [].to_vec();
    for key in root.tree_names().iter() {
        let name = str::from_utf8(key).unwrap();
        if !name.starts_with("s5name") && name != "__sled__default"{
            names.push(name.to_string());
        }
        else{

        };
    }
    names
}
/// Removes all trees in a db. Careful with that axe, Eugene.
pub fn remove_service_trees() -> bool {
    let root = get_root(JulieDatabase::Service).unwrap();
    for key in root.tree_names().iter() {
        let index = str::from_utf8(key).unwrap();
        let tree = get_tree(root.clone(),index).unwrap();
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
    // use std::env;

    #[test]
    fn sled_implementation() {
        // let config = SledConfig{
        //     db: "client".to_string()
        // };
        // println!("{:?}",config);
        let mut root = SledDb::init(JulieDatabase::Client).unwrap();
        let new_client = ClientAuth::new();
        let status = root.create(JulieDatabase::Client, (new_client.clone(), ServiceIdentity::dummy())).unwrap();
        assert!(status);
        let index = new_client.clone().uid;
        let mut client = root.read(JulieDatabase::Client, &index).unwrap().0;
        println!("{:?}",client);
        let old_email = client.email;
        client.email = "test@auth.com".to_string();
        assert!(root.update(JulieDatabase::Client, (client.clone(),ServiceIdentity::dummy())).unwrap());
        let updated = root.read(JulieDatabase::Client, &index).unwrap().0;
        println!("{:?}",updated);
        assert_ne!(old_email,updated.email)


    }
    #[test] #[ignore]
    fn delete_all_clients(){
        let status = remove_client_trees();
        assert!(status);
        assert_eq!(get_uid_indexes().len(),0);

    }
    #[test] #[ignore]
    fn delete_all_services(){
        let status = remove_service_trees();
        assert!(status);
        assert_eq!(get_uid_indexes().len(),0);

    }
}

