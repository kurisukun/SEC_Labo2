use crate::db::database::create_user;
use argon2::Config;
use rand::prelude::*;
use rusqlite::Error;

pub fn register(username: &str, password: &str) -> Result<(), Error> {
    let argon2_config = Config::default();
    let mut salt = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);

    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, &argon2_config).unwrap();

    match create_user(username, hashed_password.as_str()) {
        Ok(_) => {
            println!("User created!");
            Ok(())
        }
        Err(e) => {
            println!("Something went wrong during registration!");
            Err(e)
        }
    }
}
