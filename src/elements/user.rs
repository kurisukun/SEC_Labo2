#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

    pub fn get_username(&self) -> &String{
        &self.username
    }

    pub fn get_password(&self) -> &String{
        &self.password
    }

    pub fn get_two_factors(&self) -> bool{
        self.two_factors
    }

    pub fn get_google_token(&self) -> &String{
        &self.google_token
    }
}
