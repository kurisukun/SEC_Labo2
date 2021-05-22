use crate::db::database::update_user_secret;
use crate::validation::validation::syntatic_validation_google_token;
use crate::Errors;
use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use read_input::prelude::*;

pub fn verify_secret(secret: &str) -> bool {
    let google_auth = GoogleAuthenticator::new();

    let input_token: String = input()
        .repeat_msg("Please enter your generated token: ")
        .add_err_test(
            |m: &String| syntatic_validation_google_token(m),
            "Error: the format is not respected (only 6 numbers) ",
        )
        .get();

    //println!("SECRETS: {} {} {}", secret, input_token, google_auth.verify_code(secret, input_token.as_str(), 0, 0));
    google_auth.verify_code(secret, input_token.as_str(), 0, 0)
}

pub fn change_two_factors(username: &str, two_factors_is_enabled: bool) -> Result<bool, Errors> {
    if two_factors_is_enabled {
        return disable_two_factors(username);
    }

    enable_two_factors(username)
}

fn enable_two_factors(username: &str) -> Result<bool, Errors> {
    println!("\n### Enabling the two factors ###");

    let google_auth = GoogleAuthenticator::new();
    let secret = google_auth.create_secret(32);
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
