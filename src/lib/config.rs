use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize,Deserialize};

use crate::lib::rsa;

pub const STORAGE_ROOT: &str = ".julie";
pub const KEYS: &str = ".keys";
pub const CONFIG: &str = "config.json";
pub const KEY_NAME: &str = "julie_rsa";


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct JulieConfig{
    pub caretaker: String,
    pub host: String,
    pub port: String,
    pub log_level: String,
    pub public_key: String  
}

impl JulieConfig{
    pub fn init()->Self{
        let config_file = format!("{}/{}/{}", env::var("HOME").unwrap(), STORAGE_ROOT, CONFIG);
        let mut file = File::open(config_file).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
    
        // let json = Json::from_str(&data).unwrap();
        let json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
        
        let public_key = match read_public_key(){
            Some(key)=>key,
            None=>{
                rotate_signing_key();
                read_public_key().unwrap()
            }
        };
        JulieConfig{   
            caretaker: json.get("caretaker").unwrap().to_string(),
            host: json.get("host").unwrap().to_string(),
            port: json.get("port").unwrap().to_string(),
            log_level: json.get("log_level").unwrap().to_string(),
            public_key: public_key
        }
    }
}


pub fn rotate_signing_key(){
    let filename = format!("{}/{}/{}/", env::var("HOME").unwrap(), STORAGE_ROOT, KEYS);
    rsa::create_file(&filename,KEY_NAME);
}

pub fn read_public_key()->Option<String>{
    let filename = format!("{}/{}/{}/{}.pub", env::var("HOME").unwrap(), STORAGE_ROOT, KEYS, KEY_NAME);
    let contents = match fs::read_to_string(filename){
        Ok(result)=>result,
        Err(_)=>{
            return None
        }
    };
    Some(contents)
}

