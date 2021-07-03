use hashicorp_vault::client::VaultClient;

use crate::storage::interface::{JulieStorage, JulieDatabase};
use crate::auth::client::{ClientAuth,AuthFactor};
use crate::auth::service::{ServiceIdentity};

pub type VaultStorage = VaultClient;
const VAULT_ADDRESS: String = "https://vault.stackmate.net/v1";
const VAULT_TOKEN: String = "";
impl JulieStorage for VaultStorage{
    fn init(db: JulieDatabase) -> Result<Self, String>{
        Err("Not imlemented")
    }
    fn create(&self, db: JulieDatabase, object: (ClientAuth,ServiceIdentity)) -> Result<bool, String>{
        Err("Not imlemented")
    }
    fn read(&self,db: JulieDatabase, index: &str)-> Result<(ClientAuth,ServiceIdentity),String>{
        Err("Not imlemented")
    }
    fn update(&self,db: JulieDatabase, object: (ClientAuth,ServiceIdentity)) -> Result<bool, String>{
        Err("Not imlemented")
    }
    fn delete(&self, index: &str)-> Result<bool,String>{
        Err("Not imlemented")
    }
}