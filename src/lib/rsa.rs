extern crate openssl;

use std::fs;
use openssl::rsa::{Rsa};
use openssl::sign::{Signer, Verifier};
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;

pub fn create_file(path: &str,name: &str) {
    let rsa = Rsa::generate(4096).unwrap();
    // let keypair = PKey::from_rsa(rsa).unwrap();
    // let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    // println!("Private key: {}", String::from_utf8(private_key.clone()).unwrap());
    // println!("Public key: {}", String::from_utf8(public_key.clone()).unwrap());
    let mut full_pub_path = path.to_owned();
    full_pub_path.push_str(name);
    full_pub_path.push_str(".pub");

    let mut full_priv_path = path.to_owned();
    full_priv_path.push_str(name);
    full_priv_path.push_str(".pem");
    println!("{}",full_pub_path.clone());
    fs::write(full_pub_path, String::from_utf8(public_key).unwrap()).expect("Unable to write file");
    fs::write(full_priv_path, String::from_utf8(private_key).unwrap()).expect("Unable to write file");

}
pub fn sign(message: &str, private_key: &str)-> String{
    let keypair = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();
    let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(message.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    println!("{}", base64::encode(&signature));
    base64::encode(&signature)

}
pub fn verify(message: &str, signature: &str, public_key: &str)->bool{
    let keypair = Rsa::public_key_from_pem(public_key.as_bytes()).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();

    let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
    verifier.update(message.as_bytes()).unwrap();
    let decoded = base64::decode(signature).unwrap();
    verifier.verify(&decoded).unwrap()
}
// pub fn encrypt(message: String, public_key: String)->String{}
// pub fn decrypt(message:String, private_key: String)->String{}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn rsa_create_file() {
        let base_path = env::var("HOME").unwrap() + "/";
        let name = "test";
        create_file(&base_path.clone(), name);

        let mut full_pub_path = base_path.to_owned();
        full_pub_path.push_str(name);
        full_pub_path.push_str(".pub");

        let mut full_priv_path = base_path.to_owned();
        full_priv_path.push_str(name);
        full_priv_path.push_str(".pem");

        // println!("{}", full_priv_path);

        let pub_exists = std::path::Path::new(full_pub_path.as_str()).exists();
        let priv_exists = std::path::Path::new(full_priv_path.as_str()).exists();
        assert_eq!(pub_exists,true);
        assert_eq!(priv_exists,true);
        fs::remove_file(full_pub_path.as_str()).expect("Could not delete public key file");
        fs::remove_file(full_priv_path.as_str()).expect("Could not delete private key file");


    }

    #[test]
    fn rsasign(){

        let message = "timestamp";
        let private_key = "-----BEGIN RSA PRIVATE KEY-----\n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\n-----END RSA PRIVATE KEY-----";
        let expectedsignature = "Sgu/A0rs+LBv1//uNdtEfFD2FJ/f1olgGfhyFBGkUydyTVre04eHYIxVzjlaFKfdSIDZeTsZoYigSiW1mmLb2HjOGwkyHrofGo6QZ3hjfalAuRrKiEMgN3qzNPieN6DgU7pjXImZGv7Aejbgk8waLajnuFhGyCHA0lBp9+4L6C7ILq3WDls93GHGoTKLtYgjQL+d6ej1rOzPxCE5GphkkHsYQGL6WpaRE3PG7u8i6HAs3l8NiKTT1SN/0UQN3+73/BhfBW5ijhWVGYof/vo4Ex85/Sxdg2dT89HKqNybnJVbj3ufaJ0y9hSza1FgR41q6kwyi+Zn5OzvaF6AgiX0zLa5OnZGr8d26ULJARNweSmDMY00s+cwrTV+VLMQugQn5G0vqpO4AuKgMBRZHz0hOHXj6bNNt7NT+1VOLNaBnBZkoqy4NIwXygITWCPLNBCP1ayhftoU240fY9ETZfwnWy0m3l8Fo8tTyJZjRiu3VUHT8Cos5Tf8ZtG+OdhLX5jO5VFkAUDPlRUe+mg7bnHA3B3Yw1TeRuUjtwAAxZK7aby7OIJmtCogkZdbxlU7ny5gtVa0uEZWEYAsa3W/7xi7NSm3zzg0x3hmHpHefbB2YJbDgAzdXL1iSClrXACDt8pbRRwA7YDpyg80ze7ycGqbTP5/B/mtMwTjwzy5tTpF4kA=";
        let signature = sign(message, private_key);
        // print!("{}",signature);
        assert_eq!(signature,expectedsignature);

    }

    #[test]
    fn rsa_verify(){

        let message = "supersecret";
        let signature = "CHq0hwIxpejMqWrhMQ0SGoHwYMQJ8il0kza+r5MtlF9wMI6vpHU8/KI9+CAdkjs2QH2NkOBF6I+tKekAuzYOitnASrpz/4KmnF3FFQk/JChshNkOkyQe97wEaEcfb+/MV4llwdo659hAGIbLq8yzzLEB06uRXiaybJ/BzRzkvx2K/vMj6UuA9XFgO9DDjUko7JBtQNR9Gop4Gw9TQlk5cE+xqdj+dd7aujEeKNpWiF0knNZJC60gFyUhKWQlse7L3nqBMQV/ykwtSUMVN8wikV3HuCornnoyQnZTUc2fBm2u+mB0LuU+NphW9rlA9dsaIyMXFvDLykQqxDCtGsjQvw==";
        let public_key = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvxHJVaUo8J8PbKfpZ7lW\nffrRaSOcVmraoMwJeiYi2A/YTB2BnJNGUZd20Gj/ShRasnCWvjtZCEPpkNMAkXN4\nG7/xALKDzC9YBJCDhEP48J0sxhN37zvjlDaav2If0H/jSqKn5KlJ1im/maZ25sdG\ngYUN7h5F3kxwTBVDomTj4unmmiPiqk+/JJFq0ZMOSA+iAW4UVq14f4lTsyQ+peEe\nP2cr75Jn9/z0mTan/buYJ+B5KrM5pYLnfOKZrac2GJUzL2asJpj/ZMYyOq5YMGEk\nN29GasGlrF0unZI1P+4tmfE0j+A6Gi3MN1ZpHmFvyF2oBpLhReh7f0efvXYAemSV\nRQIDAQAB\n-----END PUBLIC KEY-----";
        let status = verify(message, signature, public_key);
        assert_eq!(status,true);

    }



}
