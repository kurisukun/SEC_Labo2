use google_authenticator::GoogleAuthenticator;

pub fn gen_secret() -> String{
    let google_auth = GoogleAuthenticator::new();
    google_auth.create_secret(32)
}

pub fn verify_secret(secret: &str, code: &str) -> bool{
    let google_auth = GoogleAuthenticator::new();
    google_auth.verify_code(secret, code, 60, 0)
}