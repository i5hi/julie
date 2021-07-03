use crate::auth::client::{ClientAuth};
use crate::auth::service::{ServiceIdentity};

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

#[derive(Debug)]
pub enum JulieDatabaseItem{
    Client(ClientAuth),
    Service(ServiceIdentity)
}


pub trait JulieStorage: Sized + Clone + Send {
    fn init(db: JulieDatabase) -> Result<Self, String>;
    fn create(&mut self, object: JulieDatabaseItem) -> Result<bool, String>;
    fn read(&mut self,db: JulieDatabase, index: &str)-> Result<JulieDatabaseItem,String>;
    fn update(&mut self, object: JulieDatabaseItem) -> Result<bool, String>;
    fn delete(&mut self, index: &str)-> Result<bool,String>;
}


