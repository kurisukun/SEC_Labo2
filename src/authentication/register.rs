use crate::errors::Errors;
use crate::validation::validation::is_secure_password;
use crate::{
    db::database::create_user,
    validation::validation::{syntatic_validation_password, syntatic_validation_username},
};
use argon2::Config;
use rand::prelude::*;


/// Proceeds to the registration of the user. The username (email) must not be already taken and
/// bother username and password have to respect a certain format (see validation.rs for more info)
///
/// Returns Result<()> if the login is done correctly, Result<Errors> otherwise
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

#[cfg(test)]
mod test {

    use super::*;
    use crate::{db::database::{establish_connection, create_table}};

    fn drop_table() {
        let conn = establish_connection();
        conn.execute("DROP TABLE users", []).unwrap();
    }

    #[test]
    fn valid_register(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let password = "aA1_lpa23B";
        let registration = register(username, password);
        assert_eq!(registration, Ok(()));
    }

    #[test]
    fn invalid_register(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let password = "password1234";
        let registration = register(username, password);
        assert_eq!(registration, Err(Errors::PasswordFormatError));
        let password = "Pa$$w0rDDD";
        let registration = register(username, password);
        assert_eq!(registration, Err(Errors::TooWeakPasswordError));
    }
}