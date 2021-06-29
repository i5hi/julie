use sha2::{Sha256, Digest};

use data_encoding::HEXLOWER_PERMISSIVE;
// use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

pub fn salted512(password: &str, salt: &str) -> String {
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt.as_bytes(),
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    HEXLOWER_PERMISSIVE.encode(&pbkdf2_hash)
}

pub fn create_salt()->String{
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt).unwrap();
    HEXLOWER_PERMISSIVE.encode(&salt)
}

pub fn sha256(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message);
    let result = hasher.finalize();
    let string = format!("{:x}", result);
    String::from(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_composite() {
        assert_eq!(sha256("karan"), String::from("46ed260db5a4cb33871f0b308aae3e899602cd7f20c6841677e4079d8b9e5ec3"));
    }

    #[test]
    fn salted512_composite(){
        let salt = create_salt();
        println!("Salt: {}",salt);
        let password = "meunhashedpass";
        let hash = salted512(password.clone(), &salt.clone());
        println!("Hash: {}",hash.clone());
        assert_eq!(hash.as_bytes().len(),128);
    }

}
