extern crate lettre;
extern crate lettre_email;

use lettre::smtp::SmtpClient;
use lettre_email::EmailBuilder;
use crate::lib::email::lettre::Transport;
use lettre::Envelope;
use lettre::EmailAddress;
use lettre::SendableEmail;

pub fn sendmail(email: &str, alias: &str, message: &str)->bool{
    let to = EmailAddress::new(email.to_string()).unwrap();
    let from = EmailAddress::new("julie@stackmate.net".to_string()).unwrap();
    let envelope = Envelope::new(Some(from),vec![to]).unwrap();

    let email = SendableEmail::new(envelope,"ID".to_string(), message.as_bytes().to_vec());
    
    let mut mailer = SmtpClient::new_unencrypted_localhost()
        .unwrap()
        .transport();
    
    let result = mailer.send(email);
    println!("{:#?}",result);
    result.is_ok()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendmail() {        
        let email = "vishalmenon.92@gmail.com";
        let alias = "Vishal Menon";
        let message = "supersecretauthtoken";
        assert!(sendmail(email, alias, message))
    }

}
