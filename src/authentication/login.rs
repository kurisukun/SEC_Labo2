use two_factors::verify_secret;

use crate::authentication::two_factors;
use crate::db::database::get_user;
use crate::elements::user::User;
use crate::errors::Errors;


/// Proceeds to the login of the user
///
/// Returns Result<User> if the login is done correctly, Result<Errors> otherwise
pub fn login(username: &str, password: &str) -> Result<User, Errors> {
    
    let result = get_user(username);
    if let Err(e) = result {
        //Will create an error but avoids timing attacks
        let _e = argon2::verify_encoded("", password.as_bytes());
        return Err(e);
    }

    let user = result.unwrap();
    let verification = argon2::verify_encoded(user.get_password().as_str(), password.as_bytes());
    //Checks if an error occurs with argon2
    if let Err(_) = verification {
        return Err(Errors::LoginError);
    }

    //Checks that the result is not false, otherwise the password does not correspond
    if !verification.unwrap(){
        return Err(Errors::LoginError);
    }

    if user.get_two_factors() && !verify_secret(user.get_google_token().as_str()) {
        return Err(Errors::TokenError);
    }

    Ok(user)
}


#[cfg(test)]
mod test {

    use super::*;
    use crate::{authentication::register::register, db::database::{establish_connection, create_table}};
    use crate::elements::user::User;

    fn drop_table() {
        let conn = establish_connection();
        conn.execute("DROP TABLE users", []).unwrap();
    }

    #[test]
    fn valid_login(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let password = "aA1_lpa23B";
        let _registration = register(username, password);
        let user = get_user(username).unwrap();
        let result = login(username , password);
        assert_eq!(result, Ok(User::new(username.to_string(), user.get_password().to_string(), false, "".to_string())));
    }

    #[test]
    fn invalid_login_user_does_not_exist(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let password = "aA1_lpa23B";
        let result = login(username, password);
        assert_eq!(result, Err(Errors::LoginError));
    }

    #[test]
    fn invalid_login_wrong_password(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let password = "aA1_lpa23B";
        let _registration = register(username, password);
        let result = login(username , "WrongPassword");
        assert_eq!(result, Err(Errors::LoginError));
    }
}