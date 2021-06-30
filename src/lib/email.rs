extern crate lettre;

use lettre::{Message, Transport, SmtpTransport};

const JULIE_EMAIL: &str = "admin@test.satswala.com";

pub fn sendmail(to: &str, alias: &str, message: &str)->bool{

    let message = Message::builder()
        .from(format!("Julie <{}>",JULIE_EMAIL).parse().unwrap())
        .to(format!("{} <{}>",alias,to).parse().unwrap())
        .subject("Woof Woof")
        .body(message.to_string())
        .unwrap();

    let transport = SmtpTransport::unencrypted_localhost();

    let result = transport.send(&message);
    result.is_ok()
    // true

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendmail() {        
        let to = "vishalmenon.92@gmail.com";
        let alias = "vmd";
        let message = "https://test.satswala.com/julie?token=supermostsecrettokenforyoumyfriendlyboi";
        assert!(sendmail(to, alias, message))
    }

}
