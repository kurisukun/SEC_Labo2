use crate::db::database::update_user_secret;
use crate::validation::validation::syntatic_validation_google_token;
use crate::Errors;
use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use read_input::prelude::*;

pub fn verify_secret(secret: &str) -> bool {
    let google_auth = GoogleAuthenticator::new();

    let input_token: String = input()
        .repeat_msg("Please enter your two factors authentication token: ")
        .add_err_test(
            |m: &String| syntatic_validation_google_token(m),
            "Error: the format is not respected (only 6 numbers) ",
        )
        .get();

    google_auth.verify_code(secret, input_token.as_str(), 0, 0)
}

pub fn change_two_factors(username: &str, two_factors_is_enabled: bool) -> Result<bool, Errors> {
    if two_factors_is_enabled {
        return disable_two_factors(username);
    }

    enable_two_factors(username)
}

fn gen_secret() -> String{
    let google_auth = GoogleAuthenticator::new();
    google_auth.create_secret(32)
}

fn enable_two_factors(username: &str) -> Result<bool, Errors> {
    println!("\n### Enabling the two factors ###");

    let google_auth = GoogleAuthenticator::new();
    let secret = gen_secret();
    let qr_code = google_auth.qr_code_url(
        secret.as_str(),
        "qr_code",
        "sec_labo2_account",
        400,
        400,
        ErrorCorrectionLevel::High,
    );

    println!("Or go to this link and scan the QR Code: {}", qr_code);
    update_user_secret(username, true, secret.as_str())
}

fn disable_two_factors(username: &str) -> Result<bool, Errors> {
    println!("\n### Disabling the two factors ###");
    update_user_secret(username, false, "")
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::db::database::{establish_connection, create_table, create_user};


    fn drop_table() {
        let conn = establish_connection();
        conn.execute("DROP TABLE users", []).unwrap();
    }


    fn create_user_for_tests(username: &str, password: &str){
        drop_table();
        create_table();
        let _result = create_user(username, password);
    }

    #[test]
    fn valid_gen_secret(){
        let secret = gen_secret();
        assert_eq!(secret.len(), 32)
    }

    #[test]
    fn invalid_gen_secret(){
        let secret = gen_secret();
        assert_ne!(secret.len(), 0);
        assert_ne!(secret.len(), 33);
        assert_ne!(secret.len(), 31);        
    }

    #[test]
    fn valid_diable_two_factors(){
        let username = "test@test.com";
        let password = "1234";
        let two_factors = false;
        let secret = "";
        create_user_for_tests(username, password);
        let _result = update_user_secret(username, two_factors, secret);
        let disabled = disable_two_factors(username).unwrap();

        assert_eq!(disabled, true);
    }

}