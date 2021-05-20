
#[derive(Debug)]
pub struct User {
    id: u32,
    username: String,
    password: String,
    two_factors: bool,
}

impl User{

    pub fn new(id: u32, username: String, password: String, two_factors: bool) -> Self{
        Self{id, username, password, two_factors}
    }
}