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

const SMTP_USER: &str = "YOUR EMAIL HERE";
const SMTP_PASS: &str = "YOUR PASSWORD HERE";
const SMTP_SERV: &str = "smtp.googlemail.com";
const MAIL_FROM: &str = "Chris Barros Henriques <chris.barroshenriques@gmail.com>";

/// Verifies that the user has taken less than 15 minutes to enter the reset token
///
/// Returns Result<()> if the duration is lower, Result<Errors> otherwise
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

/// Send by email the token for reseting the password of the account
///
/// Returns true of the mail has been sent, false otherwise
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

/// Changes the password of the user after reset token has been validated
/// The user will be asked to enter his password twice to do so and have to be the same
///
/// Returns Result<()> if the password has been changed, Result<Errors> otherwise
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
