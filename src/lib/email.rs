extern crate lettre;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use lettre::{Message, Transport, SmtpTransport};
use lettre::message::{ MultiPart};

use crate::lib::database::STORAGE_ROOT;

const JULIE_EMAIL: &str = "admin@test.satswala.com";

pub fn send(to: &str, alias: &str, message: &str)->bool{

    let message = Message::builder()
        .from(format!("Julie <{}>",JULIE_EMAIL).parse().unwrap())
        .to(format!("{} <{}>",alias,to).parse().unwrap())
        .subject("Complete Email Authentication")
        .multipart(MultiPart::alternative_plain_html(
            String::from(message),
            String::from(message),
        )).unwrap();


    let transport = SmtpTransport::unencrypted_localhost();

    let result = transport.send(&message);
    result.is_ok()
    // true

}

pub fn readHTML()->String{
    let config_file = format!("{}/{}/{}", env::var("HOME").unwrap(), STORAGE_ROOT, "email.html");
    let mut file = File::open(config_file).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data.clone()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[ignore]
    fn test_sendmail() {        
        let to = "vishalmenon.92@gmail.com";
        let alias = "vmd";
        let message = "https://test.satswala.com/julie?token=supermostsecrettokenforyoumyfriendlyboi";
        assert!(send(to, alias, message));
        let message = readHTML();
        assert!(send(to, alias, &message))
    }

}
