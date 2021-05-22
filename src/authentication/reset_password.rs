
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::time::Instant;
use uuid::Uuid;
use read_input::prelude::*;

use crate::Errors;
use crate::validation::validation::{syntatic_validation_uuid, syntatic_validation_username};

const SMTP_USER: &str = "chris.barroshenriques@gmail.com";
const SMTP_PASS: &str = "3^VCfVH8R7km%p4D*T^f";
const SMTP_SERV: &str = "smtp.googlemail.com";
const MAIL_FROM: &str = "Chris Barros Henriques <chris.barroshenriques@gmail.com>";

pub fn check_email_duration() -> Result<(), Errors>{
    const MAX_TIME_SECS: u64 = 15*60;

    let token = Uuid::new_v4().to_hyphenated().to_string();
    let start = Instant::now();

    let email = input()
        .msg("Please enter a email for the confirmation (it can be different from the one of your account): ")
        .add_err_test(move |m: &String| syntatic_validation_username(m), "Error: invalid email")
        .get();


    if !send_mail(&email, &token){
        return Err(Errors::EmailNotSentError);
    }
    println!("An email has been sent to the address: {} \n
    Please enter your generated token for password reset: ", email);

    let input : String = input()
        .repeat_msg("Please enter your generated token for password reset: ")
        .add_err_test(|m: &String| syntatic_validation_uuid(m), "Error: the format is not respected (UUID) " )
        .get();
    
    let duration = start.elapsed();

    if duration.as_secs() >= MAX_TIME_SECS {
        return Err(Errors::EmailTimeoutError);
    }
    else if input != token{
        return Err(Errors::EmailWrongToken);
    }

    Ok(())
}

fn send_mail(dst: &str, message: &str) -> bool {
    
    let email = Message::builder()
        .from(MAIL_FROM.parse().unwrap())
        .reply_to(MAIL_FROM.parse().unwrap())
        .to(dst.parse().unwrap())
        .subject("Changement de mot de passe")
        .body(format!(
            "Votre token pour la validation du changement de mot de passe : {}",
            message
        ))
        .unwrap();
    let creds = Credentials::new(SMTP_USER.to_string(), SMTP_PASS.to_string());

    let mailer = SmtpTransport::relay(SMTP_SERV)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Un mail vous a été envoyé avec un token."),
        Err(e) => {
            println!("Le mail n'a pas pu être envoyé : {:?}", e);
            return false;
        }
    }
    true
}