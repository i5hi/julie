use oath::{totp_raw_now, HashType};

pub fn generate_otp(b32_key: String, algo: HashType)->u64{
    let raw_key = base32::decode(base32::Alphabet::RFC4648 {padding: false},&b32_key).unwrap();
    totp_raw_now(&raw_key, 6, 0, 30, &algo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::aes::{keygen,Encoding};

    #[test]
    fn totp_composite(){
        let key = keygen(Encoding::Base32);
        // println!("{:#?}",key.clone());
        let raw_key = base32::decode(base32::Alphabet::RFC4648 {padding: false},&key.clone()).unwrap();
        // 256 bit key =  Vec[u8;32] 32*8 
        assert_eq!(raw_key.clone().len(),32);
        let key = "4FECSI7YPTOHGYOBCVIVO7IF5QGQGDS4EUTYOOEZLKK3ENBA3KPA".to_string();
        // println!("{:#?}",key.clone());

        // Google Authenticator defaults to SHA1
        let otp = generate_otp(key.clone(), HashType::SHA1);
        println!("{:#?}", otp.clone());
    }
}