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

pub const STORAGE_ROOT: &str = ".julie"; // Database
pub const CLIENT: &str = "client"; // Collection
pub const SERVICE: &str = "service";

/// Retrieves the primary data store @ $HOME/.satsbank. Database + Collection in mongo. 
pub fn get_root(db: &str) -> Result<Db, String> {
    let db_storage_path: String =
        format!("{}/{}/{}", env::var("HOME").unwrap(), STORAGE_ROOT, db).to_string();
    match sled::open(db_storage_path.clone()) {
        Ok(db) => Ok(db),
        Err(_) => Err(format!("E:DB Open @ {} FAILED.", db_storage_path).to_string()),
    }
}

/// Retrieves a specific tree from the root. Document.
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
    fn db_composite() {
        let index = "s5idsatswala9010";
        let root = get_root(CLIENT).unwrap();
        
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

