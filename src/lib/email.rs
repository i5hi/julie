extern crate lettre;
extern crate lettre_email;

use lettre::smtp::{SmtpClient};
use lettre_email::EmailBuilder;
use crate::lib::email::lettre::Transport;
use lettre::Envelope;
use lettre::EmailAddress;
use lettre::SendableEmail;

const JULIE_EMAIL: &str = "julie@satswala.com";

pub fn sendmail(email: &str, alias: &str, message: &str)->bool{
    let to = EmailAddress::new(email.to_string()).unwrap();
    let from = EmailAddress::new(JULIE_EMAIL.to_string()).unwrap();
    let envelope = Envelope::new(Some(from),vec![to]).unwrap();

    let email = SendableEmail::new(envelope,"MessageID".to_string(), message.as_bytes().to_vec());
    // let email = EmailBuilder::new()
    //     .to((email, alias))
    //     .from(JULIE_EMAIL)
    //     .subject("Woofy")
    //     .text("supersecretokensauces")
    //     .build()
    //     .unwrap();

    let mut mailer = SmtpClient::new_unencrypted_localhost()
        .unwrap()
        .transport();
    // let mut mailer = SmtpTransport::builder_unencrypted_localhost().unwrap();
                             
    let result = mailer.send(email);
    println!("{:#?}",result);
    result.is_ok()
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
