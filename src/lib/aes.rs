// use std::iter::repeat;
use openssl::symm;
use std::str;
use rand::prelude::*;
// extern crate hex;


pub enum Encoding {
    Base64,
    Base32,
    Hex
}
// use rand_seeder::{Seeder};
// use rand_pcg::Pcg64;

/// Create a random 256 bit key to use for aes encryption
pub fn keygen(encoding: Encoding)->String{
    let key: Vec<u8> =rand::thread_rng().gen::<[u8; 32]>().to_vec();
    match encoding  {
        Encoding::Base64=>base64::encode(key),
        Encoding::Base32=>base32::encode(base32::Alphabet::RFC4648 {padding: false}, &key),
        Encoding::Hex=>hex::encode(key)
    }

}

/// Create a seeded 256 bit key to use for aes encryption
// pub fn seedgen(seed:String)->String{
//     let mut key: Pcg64 = Seeder::from(seed.as_str()).make_rng();
//     base64::encode(key.gen::<[u8; 32]>())
// }

/// String wrapper for AES-256-CBC encrypt 
pub fn encrypt(plaintext:&str, key: &str)->String{
    let iv = rand::thread_rng().gen::<[u8; 16]>().to_vec();
    let cipher = symm::Cipher::aes_256_cbc();
    let ciphertext = symm::encrypt(
        cipher,
        &base64::decode(key).unwrap(),
        Some(&iv),
        plaintext.as_bytes()
    ).unwrap();
    base64::encode(iv)+ &String::from(":") + &base64::encode(ciphertext).to_string()

}

/// String wrapper for AES-256-CBC decrypt 
pub fn decrypt(iv_ciphertext:&str, key: &str)->String{
    let cipher = symm::Cipher::aes_256_cbc();
    let iter:Vec<&str> = iv_ciphertext.split(":").collect();
    let plaintext = symm::decrypt(
        cipher,
        &base64::decode(key).unwrap(),
        Some(&base64::decode(iter[0]).unwrap()),
        &base64::decode(iter[1]).unwrap()
    ).unwrap();

    str::from_utf8(&plaintext).unwrap().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen(){
        println!("FRESH RANDOM KEY:  {}  :: length={}",keygen(Encoding::Base64), keygen(Encoding::Base64).len());
        println!("FRESH RANDOM KEY:  {}  :: length={}",keygen(Encoding::Hex), keygen(Encoding::Hex).len())

    }

    // #[test]
    // fn test_seedgen(){
    //     println!("SEEDED KEY:  {}",seedgen(String::from("myseed")))
    // }

    #[test]
    fn test_encrypt_decrypt(){
        let secret = "I am very much interested torustyou";
        let key = "a79FAWI1IKtuwoSoT3hq0lfkq0oxchoHy1xhOTSpHaU=";
        let iv_ciphertext = encrypt(secret.clone(),key.clone());
        println!("IV ENCRYPTED SECRET:  {}",&iv_ciphertext);
        let plaintext = decrypt(&iv_ciphertext.clone(), key.clone());
        println!("IV DECRYPTED SECRET:  {}",&plaintext);
        assert_eq!(secret,plaintext)
    }
}
