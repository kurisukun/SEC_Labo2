use crate::errors::Errors;
use crate::validation::validation::is_secure_password;
use crate::{
    db::database::create_user,
    validation::validation::{syntatic_validation_password, syntatic_validation_username},
};
use argon2::Config;
use rand::prelude::*;

pub fn register(username: &str, password: &str) -> Result<(), Errors> {
    if !syntatic_validation_username(username) {
        return Err(Errors::EmailFormatError);
    }

    if !syntatic_validation_password(password) {
        return Err(Errors::PasswordFormatError);
    }

    if !is_secure_password(password) {
        return Err(Errors::TooWeakPasswordError);
    }

    let argon2_config = Config::default();
    let mut salt = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);

    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, &argon2_config).unwrap();

    match create_user(username, hashed_password.as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
