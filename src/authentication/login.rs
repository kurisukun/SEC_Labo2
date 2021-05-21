use rusqlite::Error;
use two_factors::verify_secret;

use crate::db::database::get_user;
use crate::authentication::two_factors;

pub fn login(username: &str, password: &str, google_token: &str) -> Result<bool, Error>{

    let result = get_user(username);
    if let Err(e) = result{
        return Err(e);
    }

    let user = result.unwrap();

    //TODO change error type
    if let Err(e) = argon2::verify_encoded(user.get_password().as_str(), password.as_bytes()) {
        return Err(Error::InvalidQuery);
    }


    //TODO change error type
    if user.get_two_factors() && !verify_secret(user.get_google_token().as_str(), google_token){
        return Err(Error::InvalidQuery);
    }

    Ok(true)
}
