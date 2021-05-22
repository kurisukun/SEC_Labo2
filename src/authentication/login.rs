use two_factors::verify_secret;

use crate::authentication::two_factors;
use crate::db::database::get_user;
use crate::elements::user::User;
use crate::errors::Errors;
use google_authenticator::GoogleAuthenticator;

pub fn login(username: &str, password: &str) -> Result<User, Errors> {
    let result = get_user(username);
    if let Err(e) = result {
        return Err(e);
    }

    let user = result.unwrap();

    if let Err(_) = argon2::verify_encoded(user.get_password().as_str(), password.as_bytes()) {
        return Err(Errors::LoginError);
    }

    let google_auth = GoogleAuthenticator::new();
    let secret = user.get_google_token();
    //println!("LOGIN: You should enter this: {}", google_auth.get_code(&secret, 0).unwrap());
    if user.get_two_factors() && !verify_secret(user.get_google_token().as_str()) {
        return Err(Errors::TokenError);
    }

    Ok(user)
}
