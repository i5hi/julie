use hashicorp_vault::client::{VaultClient,TokenData};
use hashicorp_vault::client::error::{Result as VaultResult};

use crate::storage::interface::{JulieStorage, JulieDatabase, JulieDatabaseItem};
use crate::auth::client::{ClientAuth};
use crate::auth::service::{ServiceIdentity};

#[derive(Debug)]
pub struct VaultStorage{
    http_client: VaultClient<TokenData>
}

impl Clone for VaultStorage{
    fn clone(&self) -> Self {
        let client  = VaultClient::new(&self.http_client.host, &self.http_client.token).unwrap();
        VaultStorage {
            http_client: client
        }
    }
}

// HACK ME :) 
const VAULT_ADDRESS: &str = "https://vault.stackmate.net";
const VAULT_CLIENT_TOKEN: &str = "s.IvslwhG65dfQcRigKZ8iBPT6";
const VAULT_SERVICE_TOKEN: &str = "s.focBoVGrW0iUT7HxJa0qVdIm";

impl JulieStorage for VaultStorage{
    fn init(db: JulieDatabase) -> std::result::Result<Self, String> where Self:Sized{
      
        match db{
            JulieDatabase::Client=>{
                let mut storage = VaultStorage {
                    http_client: VaultClient::new(VAULT_ADDRESS.to_string(), VAULT_CLIENT_TOKEN).unwrap()
                };
                Ok(storage)
            }
            JulieDatabase::Service=>{
                let mut storage = VaultStorage {
                    http_client: VaultClient::new(VAULT_ADDRESS.to_string(), VAULT_SERVICE_TOKEN).unwrap()
                };
                Ok(storage)
            }
        }
    }
    fn create(&mut self, object: JulieDatabaseItem) -> Result<bool, String>{
        match object{
            JulieDatabaseItem::Client(client)=>{
                self.http_client.secret_backend("julie-test-client");
                Ok(self.http_client.set_custom_secret(&client.uid,&client).is_ok())

            }
            JulieDatabaseItem::Service(service)=>{
                self.http_client.secret_backend("julie-test-service");
                Ok(self.http_client.set_custom_secret(&service.name,&service).is_ok())
            }
        }
    }
    fn read(&mut self,db: JulieDatabase, index: &str)-> Result<JulieDatabaseItem,String>{
        match db{
            JulieDatabase::Client=>{
                self.http_client.secret_backend("julie-test-client");
                let secret: VaultResult<ClientAuth> = self.http_client.get_custom_secret(index);
                if secret.is_ok() {
                    Ok(JulieDatabaseItem::Client(secret.unwrap()))
                }
                else{
                    Err("None".to_string())
                }

            }
            JulieDatabase::Service=>{
                self.http_client.secret_backend("julie-test-service");

                let secret: VaultResult<ServiceIdentity> = self.http_client.get_custom_secret(index);
                if secret.is_ok() {
                    Ok(JulieDatabaseItem::Service(secret.unwrap()))

                }
                else{
                    Err("None".to_string())
                }
            }
        }
    }
    fn update(&mut self, object: JulieDatabaseItem) -> Result<bool, String>{
        match object{
            JulieDatabaseItem::Client(client)=>{
                // let serialized = serde_json::to_string(&object.0).unwrap();
                self.http_client.secret_backend("julie-test-client");
                Ok(self.http_client.set_custom_secret(client.clone().uid,&client).is_ok())

            }
            JulieDatabaseItem::Service(service)=>{
                self.http_client.secret_backend("julie-test-service");
                Ok(self.http_client.set_custom_secret(service.clone().name,&service).is_ok())
            }
        }
    }
    fn delete(&mut self, db: JulieDatabase,index: &str)-> Result<bool,String>{
        match db {
            JulieDatabase::Client=>self.http_client.secret_backend("julie-test-client"),
            JulieDatabase::Service=>self.http_client.secret_backend("julie-test-service")
        }
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
            JulieDatabaseItem::Service(_)=>{panic!("OH NO!!! WHY DID YOU DO THIS??! :(")},
        };
        assert_eq!(read.clone().uid, client.clone().uid);
        assert_eq!(read.clone().apikey, client.clone().apikey);
        assert!(client_storage.delete(JulieDatabase::Client,&read.uid).unwrap());
        let fail_read = client_storage.read(JulieDatabase::Client,&client.clone().uid);
        match fail_read{
            Ok(_)=>{},
            Err(e)=>assert_eq!(&e,"None")

        }

    }
}