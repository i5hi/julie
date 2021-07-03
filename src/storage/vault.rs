use hashicorp_vault::*;

use crate::storage::interface::{JulieStorage, JulieDatabase};
use crate::auth::client::{ClientAuth,AuthFactor};
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
    fn create(&mut self, db: JulieDatabase, object: (ClientAuth,ServiceIdentity)) -> std::result::Result<bool, String>{
        match db{
            JulieDatabase::Client=>{
                // let serialized = serde_json::to_string(&object.0).unwrap();
                Ok(self.http_client.set_custom_secret(object.clone().0.uid,&object).is_ok())

            }
            JulieDatabase::Service=>{
                Ok(self.http_client.set_custom_secret(object.clone().1.sid,&object).is_ok())
            }
        }
    }
    fn read(&mut self,db: JulieDatabase, index: &str)-> std::result::Result<(ClientAuth,ServiceIdentity),String>{
        match db{
            JulieDatabase::Client=>{
                Err("Not implemented".to_string())
            }
            JulieDatabase::Service=>{
                Err("Not implemented".to_string())
            }
        }
    }
    fn update(&mut self,db: JulieDatabase, object: (ClientAuth,ServiceIdentity)) -> std::result::Result<bool, String>{
        match db{
            JulieDatabase::Client=>{
                Err("Not implemented".to_string())
            }
            JulieDatabase::Service=>{
                Err("Not implemented".to_string())
            }
        }
    }
    fn delete(&mut self, index: &str)-> std::result::Result<bool,String>{

        Err("Not implemented".to_string())
          
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
        let created = client_storage.create(JulieDatabase::Client, (client, ServiceIdentity::dummy())).unwrap();
        assert!(created)
    }
}