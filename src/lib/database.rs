/**
 *
 * charizard's db is configured on the file system as follows:
 *
 * $HOME/.julie/$collection/$index
 *
 * $collection is the name of the model requiring the db - relative to mongo this is the name of the collection
 * $index is the key in the model structure which is used as the index - relative to mongo this is the index(_id) to a document.
 *
 *  
 * The primary benefit of this model is ease of use and implementation.
 *
 * The primary limitation is single index per document
 * 
 * If you want to use more than one index, you will have to create an alternative index tree
 *
 * Also note if you ask sled to get_tree, if will create that index if it doesnt exist. 
 * 
 * For this reason all implementors of sled need to explicity drop() the tree if they were expecting a docuemnt and got None.
 * 
 */
use std::env;
use std::str;
// use std::fs;

use sled::{Db, Tree};

pub const STORAGE_ROOT: &str = ".julie"; // Database
pub const CLIENT: &str = "client"; // Collection
pub const SERVICE: &str = "service"; // Collection

/// Retrieves the primary data store @ $HOME/.julie. Get (Database + Collection) in mongo. 
pub fn get_root(db: &str) -> Result<Db, String> {
    let db_storage_path: String =
        format!("{}/{}/{}", env::var("HOME").unwrap(), STORAGE_ROOT, db).to_string();
    match sled::open(db_storage_path.clone()) {
        Ok(db) => Ok(db),
        Err(_) => Err(format!("E:DB Open @ {} FAILED.", db_storage_path).to_string()),
    }
}

/// Retrieves a specific tree from the selected root db. Get Document.
pub fn get_tree(root: Db, index: &str) -> Result<Tree, String> {
    match root.open_tree(index.clone().as_bytes()) {
        Ok(db) => Ok(db),
        Err(_) => Err(format!("E:Tree Open @ {} FAILED.", index).to_string()),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    // use std::env;

    #[test]
    fn sled_composite() {
        let index = "s5idsatswala9010";
        let root = get_root(CLIENT).unwrap();
        
        let tree = get_tree(root.clone(), index.clone()).unwrap();

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
        tree.flush().unwrap();
        root.drop_tree(&tree.name()).unwrap();
        root.flush().unwrap();

        
        for key in root.clone().tree_names().iter() {

            println!("Name: {:?}",str::from_utf8(&key).unwrap());
           
    
        }

    }
}

