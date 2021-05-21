mod authentication;
mod db;
mod elements;
mod validation;

use crate::db::database::{drop_table, create_table};
use authentication::register::register;
use authentication::login::login;
use structopt::StructOpt;
use validation::validation::{syntatic_validation_password, syntatic_validation_username};

#[derive(Debug)]
struct User {
    id: i32,
    email: String,
    password: String,
    two_factors: bool,
    google_token: String,
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "sec_labo2",
    about = "SEC Labo2: two-factor authentication program using Google Authenticator"
)]
struct Options {
    //arg to register
    #[structopt(short, long, help = "Argument to proceed a registration")]
    register: bool,
    //arg for username
    #[structopt(short = "u", long = "username", default_value = "")]
    username: String,
    //arg for user password
    #[structopt(short = "p", long = "password", default_value = "")]
    password: String,
    //arg to enable/disable the two_factors use
    #[structopt(
        short,
        long,
        help = "Argument to enable/disable the two factors authentication"
    )]
    two_factors: bool,
    #[structopt(short = "g", long = "token", default_value = "")]
    google_token: String,
}

fn main() {

    create_table();
    let user_options = Options::from_args();
    println!("{:?}", user_options);

    if !syntatic_validation_username(&user_options.username)
        || !syntatic_validation_password(&user_options.password)
    {
        println!("Error: credentials not valid!");
        return;
    }

    if user_options.register {
        match register(&user_options.username, &user_options.password) {
            Ok(_) => println!("Registration done! You can now connect to your account"),
            Err(e) => println!("{}", e),
        }
    } else {

        match login(&user_options.username, &user_options.password, &user_options.google_token){
            Ok(connected) => {
                println!("You are now connected!")
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
