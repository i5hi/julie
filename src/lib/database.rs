/**
 *
 * charizard's db is configured on the file system as follows:
 *
 * $HOME/.satsbank/$service/$index
 *
 * $service is the name of the service requiring the db - relative to mongo this is the name of the collection
 * $index is the value in the service structure which is used as the index - relative to mongo this is the index(_id) to a document.
 *
 * This is an architectural choice that is made for a single service model.
 *  
 * The primary benefit of this model is ease of use  and implementation.
 *
 * The primary limitation is single index per document
 */
use std::env;
use std::str;
// use std::fs;

use sled::{Db, Tree};

const STORAGE_ROOT: &str = ".satsbank";
pub const CLIENT_TREE: &str = "client";
pub const SERVICE_TREE: &str = "service";

/// Retrieves the primary data store @ $HOME/.satsbank. Database in mongo. 
pub fn get_root(service: &str) -> Result<Db, String> {
    let service_storage_path: String =
        format!("{}/{}/{}", env::var("HOME").unwrap(), STORAGE_ROOT, service).to_string();
    match sled::open(service_storage_path.clone()) {
        Ok(db) => Ok(db),
        Err(_) => Err(format!("E:DB Open @ {} FAILED.", service_storage_path).to_string()),
    }
}

/// Retrieves a specific tree from the root. Collection in mongo.
pub fn get_tree(root: Db, tree: &str) -> Result<Tree, String> {
    match root.open_tree(tree.clone().as_bytes()) {
        Ok(db) => Ok(db),
        Err(_) => Err(format!("E:Tree Open @ {} FAILED.", tree).to_string()),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    // use std::env;

    #[test]
    fn db_composite() {
        let service = "client";
        let index = "s5idsatswala9010";
        let root = get_root(service).unwrap();
        let tree = get_tree(root, index.clone()).unwrap();

        let status = tree.contains_key(b"uid");
        println!("{:#?}",status);

        match tree.get(b"uid").unwrap() {
            Some(value) => {
                println!("Found: {:#?}", str::from_utf8(&value.to_vec()).unwrap());
            }
            None => {
                println!("Error: Found None");
            }
        }

  


        // tree.insert(b"uid", index.as_bytes()).unwrap();

        tree.flush().unwrap();

    }
}


/*
pub fn _get_indexes(service: &str) -> Result<Vec<String>, String>{
    let root_path: String =
    format!("{}/{}/{}", env::var("HOME").unwrap(), _STORAGE_ROOT, service).to_string();

    let dirs = match fs::read_dir(root_path.clone()){
        Ok(result)=>result,
        Err(_)=> return Err(format!("E:Tree read @ {} FAILED.", root_path).to_string()),
    };

    let mut indexes: Vec<String>=vec![];

    for entry in dirs {
        let path = entry.unwrap().path();
        if path.is_dir(){
            indexes.push(format!("{}",path.clone().display()));
        }
    };

    Ok(indexes)

}
*/

