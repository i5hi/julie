
use sled::{Db, Tree};
use crate::storage::interface::{JulieStorage, JulieDatabase, JulieDatabaseItem, StorageBoxClone};
use crate::auth::client::{ClientAuth};
use crate::auth::service::{ServiceIdentity};


use std::env;
use std::str;

pub const STORAGE_ROOT: &str = ".julie"; // Database

#[derive(Debug)]
pub struct SledConfig{
    db: String,
}

#[derive(Debug)]
pub struct SledDb{
    db: Db
}
impl StorageBoxClone for SledDb{
    fn clone_box(&self) -> Box<dyn JulieStorage> {
        let client  = self.db.clone();
        Box::new(SledDb {
            db: client
        })
    }
}
impl JulieStorage for SledDb {

    /// Client index = uid
    /// Service index = name

    fn create(&mut self, object: JulieDatabaseItem) -> Result<bool, String> {
        match  object{
            JulieDatabaseItem::Client(client)=>{
                let main_tree = get_tree(self.db.clone(), &client.uid).unwrap();
                // TODO!!! check if tree contains data, do not insert

                let bytes = bincode::serialize(&client).unwrap();
                main_tree.insert("client", bytes).unwrap();
                Ok(true)

            }
            JulieDatabaseItem::Service(service)=>{
                let main_tree = get_tree(self.db.clone(), &service.name).unwrap();
                // TODO !!! check if tree contains data, do not insert
                let bytes = bincode::serialize(&service).unwrap();
                main_tree.insert("service", bytes).unwrap();
                Ok(true)
            }
        }
      
    }
    fn read(&mut self,db: JulieDatabase, index: &str)-> Result<JulieDatabaseItem,String>{
        match db {
            JulieDatabase::Client=>{
                match get_tree(self.db.clone(), index){
                    Ok(tree)=>{
                         if tree.contains_key(b"client").unwrap() {
                             match tree.get("client").unwrap() {
                                 Some(bytes) => {
                                     let client: ClientAuth = bincode::deserialize(&bytes).unwrap();
                                     Ok(JulieDatabaseItem::Client(client))
                                 },
                                 None => Err("No client found in database - use uid as index".to_string()),
                             }
                         } else {
                             self.db.drop_tree(&tree.name()).unwrap();
                             Err("No client found in database - use uid as index".to_string())
                         }
                    }
                    Err(_)=>{
                        Err("Could not get client tree".to_string())
                    }
                }

            }
            JulieDatabase::Service=>{
                match get_tree(self.db.clone(), index){
                    Ok(tree)=>{
                         // if this tree exists return it
                         if tree.contains_key(b"service").unwrap() {
                             match tree.get("service").unwrap() {
                                 Some(bytes) => {
                                    let service: ServiceIdentity = bincode::deserialize(&bytes).unwrap();
                                    Ok(JulieDatabaseItem::Service(service))
                                },
                                 None => Err("No service found in database - use name as index".to_string()),
                             }
                         } else {
                             self.db.drop_tree(&tree.name()).unwrap();
                             Err("No service found in database - use name as index".to_string())
                         }
                    }
                    Err(_)=>{
                        Err("Could not get service tree".to_string())
                    }
                }
            }
        }
     
    }
    fn list(&mut self, db:JulieDatabase)-> Result<Vec<String>,String>{
        return Err("None".to_string())
    }
    fn update(&mut self, object: JulieDatabaseItem) -> Result<bool, String>{
        match object {
            JulieDatabaseItem::Client(client)=>{
          
                let main_tree = get_tree(self.db.clone(), &client.clone().uid).unwrap();
        
                let bytes = bincode::serialize(&client).unwrap();
        
                main_tree.insert("client", bytes).unwrap();
                main_tree.flush().unwrap();
                Ok(true)
            }
            JulieDatabaseItem::Service(service)=>{
                let main_tree = get_tree(self.db.clone(), &service.clone().name).unwrap();
        
                let bytes = bincode::serialize(&service).unwrap();
        
                main_tree.insert("service", bytes).unwrap();
                main_tree.flush().unwrap();
                Ok(true)
            }
        }
     

    }
    fn delete(&mut self, db: JulieDatabase, index: &str)-> Result<bool,String>{
        
        let tree = get_tree(self.db.clone(), index).unwrap();
        tree.clear().unwrap();
        tree.flush().unwrap();
        self.db.drop_tree(&tree.name()).unwrap();
    
        Ok(true)

    }
}

pub fn init(db: JulieDatabase) -> Result<impl JulieStorage, String>{
    match get_root(db){
        Ok(root)=>{
            Ok(SledDb{
                db:root
            })
        },
        Err(e)=>{
            Err(e)
        }
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
    match root.open_tree(index.as_bytes()) {
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
        let mut root = init(JulieDatabase::Client).unwrap();
        let new_client = ClientAuth::new();
        let status = root.create(JulieDatabaseItem::Client(new_client.clone())).unwrap();
        assert!(status);
        let index = new_client.clone().uid;
        let mut client = match root.read(JulieDatabase::Client, &index).unwrap(){
            JulieDatabaseItem::Client(client)=>client,
            JulieDatabaseItem::Service(_)=>{panic!("OH NO! HOW COULD YOU!?!?! :(")},
        };
        println!("{:#?}",client);
        let old_email = client.email;
        client.email = "test@auth.com".to_string();
        assert!(root.update(JulieDatabaseItem::Client(client.clone())).unwrap());
        let updated = match root.read(JulieDatabase::Client, &index).unwrap(){
            JulieDatabaseItem::Client(client)=>client,
            JulieDatabaseItem::Service(_)=>{panic!("OH NO! HOW COULD YOU!?!?! :(")},
        };
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

