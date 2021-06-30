extern crate lettre;
extern crate lettre_email;

use lettre::{Message, Transport, SmtpTransport};
// use crate::lib::email::lettre::Transport;
// use lettre::Envelope;
// use lettre::EmailAddress;
// use lettre::SendableEmail;

use lettre_email::EmailBuilder;
// use lettre::transport::Transport;

const JULIE_EMAIL: &str = "admin@test.satswala.com";

pub fn sendmail(email: &str, alias: &str, message: &str)->bool{
    // let to = EmailAddress::new(email.to_string()).unwrap();
    // let from = EmailAddress::new(JULIE_EMAIL.to_string()).unwrap();
    // let envelope = Envelope::new(Some(from),vec![to]).unwrap();

    // let email = SendableEmail::new(envelope,"MessageID".to_string(), message.as_bytes().to_vec());
 

    // let mut mailer = SmtpClient::new_unencrypted_localhost()
    //     .unwrap()
    //     .transport();
    // // let mut mailer = SmtpTransport::builder_unencrypted_localhost().unwrap();
                             
    // let result = mailer.send(email.clone());
    // println!("{:?}",email.message());
    // result.is_ok()
    let email = Message::builder()
        .from(format!("Julie <{}>",JULIE_EMAIL).parse().unwrap())
        .to(format!("{} <{}>",alias,email).parse().unwrap())
        .subject("Woof Woof")
        .body("Woof Woof".to_string())
        .unwrap();

    let transport = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .build();

    let result = transport.send(&email);
    result.is_ok()
    // true

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendmail() {        
        let email = "tech@stackmate.in";
        let alias = "Vishal Menon";
        let message = "supersecretauthtoken";
        assert!(sendmail(email, alias, message))
    }

}
