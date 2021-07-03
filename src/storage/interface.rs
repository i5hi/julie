use crate::auth::client::{ClientAuth};
use crate::auth::service::{ServiceIdentity};
use serde::{Serialize, Deserialize};

use std::marker::Sized;


pub enum JulieDatabase{
    Client,
    Service
}
impl JulieDatabase{
    pub fn to_string(&self)->String{
        match self{
            JulieDatabase::Client=>"client".to_string(),
            JulieDatabase::Service=>"service".to_string()
        }
    }
    pub fn from_str(db: &str)->JulieDatabase{
        match db {
            "client"=>JulieDatabase::Client,
            "service"=>JulieDatabase::Service,
            &_=>JulieDatabase::Client
        }
    }
}

// This over generalized storage interface makes tradeoffs for its convenient usage.Deserialize
// It is up to the user of this interface to ensure that the object being passed :Deserialize
// 1. is a ClientAuth if db is Client
// 2. is a ServiceIdentity if the db is Service
// Failinig to do so will result in data loss and you will lose sats!
// Read will give back a default value in the tuple for the unselected database
pub trait JulieStorage: Sized + Clone + Send {
    fn init(db: JulieDatabase) -> Result<Self, String>;
    fn create(&mut self, db: JulieDatabase, object: (ClientAuth,ServiceIdentity)) -> Result<bool, String>;
    fn read(&mut self,db: JulieDatabase, index: &str)-> Result<(ClientAuth,ServiceIdentity),String>;
    fn update(&mut self,db: JulieDatabase, object: (ClientAuth,ServiceIdentity)) -> Result<bool, String>;
    fn delete(&mut self, index: &str)-> Result<bool,String>;
}


