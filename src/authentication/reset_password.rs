use argon2::Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::prelude::*;
use read_input::prelude::*;
use std::time::Instant;
use uuid::Uuid;

use crate::validation::validation::{
    syntatic_validation_password, syntatic_validation_username, syntatic_validation_uuid,
};
use crate::Errors;
use crate::{db::database::update_user_password, validation::validation::is_secure_password};

const SMTP_USER: &str = "chris.barroshenriques@gmail.com";
const SMTP_PASS: &str = "3^VCfVH8R7km%p4D*T^f";
const SMTP_SERV: &str = "smtp.googlemail.com";
const MAIL_FROM: &str = "Chris Barros Henriques <chris.barroshenriques@gmail.com>";

pub fn check_email_duration() -> Result<(), Errors> {
    const MAX_TIME_SECS: u64 = 15 * 60;

    let token = Uuid::new_v4().to_hyphenated().to_string();
    let start = Instant::now();

    let email = input()
        .msg("Please enter a email for the confirmation (it can be different from the one of your account): ")
        .add_err_test(move |m: &String| syntatic_validation_username(m), "Error: invalid email")
        .get();

    if !send_mail(&email, &token) {
        return Err(Errors::EmailNotSentError);
    }

    let input: String = input()
        .repeat_msg("Please enter your generated token for password reset: ")
        .add_err_test(
            |m: &String| syntatic_validation_uuid(m),
            "Error: the format is not respected (UUID) ",
        )
        .get();

    let duration = start.elapsed();

    if duration.as_secs() >= MAX_TIME_SECS {
        return Err(Errors::EmailTimeoutError);
    } else if input != token {
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

pub fn change_password(username: &str) -> Result<(), Errors> {
    let argon2_config = Config::default();
    let mut salt = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);

    let password: String = input()
        .repeat_msg("Please enter your new password: ")
        .add_err_test(
            |m: &String| syntatic_validation_password(m) && is_secure_password(m),
            "Error: password is not valid, try again",
        )
        .get();

    let again_password: String = input()
        .repeat_msg("Please enter your new password again : ")
        .add_err_test(
            |m: &String| syntatic_validation_password(m) && is_secure_password(m),
            "Error: password is not valid, try again",
        )
        .get();

    if password != again_password {
        return Err(Errors::PasswordDifferentError);
    }

    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, &argon2_config).unwrap();

    match update_user_password(username, &hashed_password) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

//TODO Tests
