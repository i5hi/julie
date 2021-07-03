use hashicorp_vault::*;

use crate::storage::interface::{JulieStorage, JulieDatabase, JulieDatabaseItem};
use crate::auth::client::{ClientAuth};
use crate::auth::service::{ServiceIdentity};

pub struct VaultStorage{
    http_client: hashicorp_vault::client::VaultClient<hashicorp_vault::client::TokenData>
}

impl Clone for VaultStorage{
    fn clone(&self) -> Self {
        let client  = hashicorp_vault::client::VaultClient::new(&self.http_client.host, &self.http_client.token).unwrap();
        VaultStorage {
            http_client: client
        }
    }
}

// HACK ME :) 
const VAULT_ADDRESS: &str = "https://vault.stackmate.net";
const VAULT_CLIENT_TOKEN: &str = "s.VPBmiWlrlHHv0K3xsEf0yPuw";
const VAULT_SERVICE_TOKEN: &str = "s.VeZgZEpErKmR1eyRHywJnD57";

impl JulieStorage for VaultStorage{
    fn init(db: JulieDatabase) -> std::result::Result<Self, String>{
        let mut client = VaultStorage {
            http_client: hashicorp_vault::client::VaultClient::new(VAULT_ADDRESS.to_string(), VAULT_CLIENT_TOKEN).unwrap()
        };
        match db{
            JulieDatabase::Client=>{
                client.http_client.secret_backend("julie/test/client");
                Ok(client)
            }
            JulieDatabase::Service=>{
                client.http_client.secret_backend("julie/test/service");
                Ok(client)
            }
        }
    }
    fn create(&mut self, object: JulieDatabaseItem) -> std::result::Result<bool, String>{
        match object{
            JulieDatabaseItem::Client(client)=>{
                Ok(self.http_client.set_custom_secret(&client.uid,&client).is_ok())

            }
            JulieDatabaseItem::Service(service)=>{
                Ok(self.http_client.set_custom_secret(&service.name,&service).is_ok())
            }
        }
    }
    fn read(&mut self,db: JulieDatabase, index: &str)-> std::result::Result<JulieDatabaseItem,String>{
        match db{
            JulieDatabase::Client=>{
                let secret: Result<ClientAuth> = self.http_client.get_custom_secret(index);
                if secret.is_ok() {
                    Ok(JulieDatabaseItem::Client(secret.unwrap()))
                }
                else{
                    Err("None".to_string())
                }

            }
            JulieDatabase::Service=>{
                let secret: Result<ServiceIdentity> = self.http_client.get_custom_secret(index);
                if secret.is_ok() {
                    Ok(JulieDatabaseItem::Service(secret.unwrap()))

                }
                else{
                    Err("None".to_string())
                }
            }
        }
    }
    fn update(&mut self, object: JulieDatabaseItem) -> std::result::Result<bool, String>{
        match object{
            JulieDatabaseItem::Client(client)=>{
                // let serialized = serde_json::to_string(&object.0).unwrap();
                Ok(self.http_client.set_custom_secret(client.clone().uid,&client).is_ok())

            }
            JulieDatabaseItem::Service(service)=>{
                Ok(self.http_client.set_custom_secret(service.clone().name,&service).is_ok())
            }
        }
    }
    fn delete(&mut self, index: &str)-> std::result::Result<bool,String>{

        Ok(self.http_client.delete_secret(index).is_ok())
         
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // use std::env;

    #[test]
    fn vault_implementation() {
        let mut client_storage = VaultStorage::init(JulieDatabase::Client).unwrap();
        let client = ClientAuth::new();
        let created = client_storage.create(JulieDatabaseItem::Client(client.clone())).unwrap();
        assert!(created);
        let read = match client_storage.read(JulieDatabase::Client,&client.clone().uid).unwrap(){
            JulieDatabaseItem::Client(client)=>client,
            JulieDatabaseItem::Service(service)=>{ClientAuth::new()},
        };
        assert_eq!(read.clone().uid, client.clone().uid);
        assert_eq!(read.clone().apikey, client.clone().apikey);
        assert!(client_storage.delete(&read.uid).unwrap());
        let fail_read = client_storage.read(JulieDatabase::Client,&client.clone().uid);
        match fail_read{
            Ok(_)=>{},
            Err(e)=>assert_eq!(&e,"None")

        }

    }
}