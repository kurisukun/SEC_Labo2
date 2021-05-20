#[derive(Debug, PartialEq, PartialOrd)]
pub struct User {
    username: String,
    password: String,
    two_factors: bool,
    google_token: String,
}

impl User {
    pub fn new(
        username: String,
        password: String,
        two_factors: bool,
        google_token: String,
    ) -> Self {
        Self {
            username,
            password,
            two_factors,
            google_token,
        }
    }
}
