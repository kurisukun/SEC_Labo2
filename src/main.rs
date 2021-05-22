mod authentication;
mod db;
mod elements;
mod errors;
mod validation;

use crate::db::database::create_table;
use crate::errors::Errors;
use authentication::{login::login, reset_password::{change_password, check_email_duration}};
use authentication::{register::register, two_factors::change_two_factors};
use structopt::StructOpt;

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
    #[structopt(
        short = "p",
        long = "password",
        default_value = "",
        help = "Verifies the syntax of password and given criterium:\n  -the size of the password is between 10-20 chars\n  -has at least one lowercase char\n  -has at least one uppercase char\n  -has at least one special char in the given list: .?!@_-#$%^&*+\n"
    )]
    password: String,
    //arg to enable/disable the two_factors use
    #[structopt(
        short,
        long,
        help = "Argument to enable/disable the two factors authentication"
    )]
    two_factors: bool,
    //arg to enable/disable the two_factors use
    #[structopt(long, help = "Argument to reset the password")]
    reset_password: bool,
}

fn main() {
    create_table();
    let user_options = Options::from_args();

    if user_options.register {
        println!("### Registration ###");
        match register(&user_options.username, &user_options.password) {
            Ok(_) => println!("Registration done! You can now connect to your account"),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("### Login ###");
        match login(&user_options.username, &user_options.password) {
            Ok(user) => {
                println!("Hi {}, you are now connected!", user.get_username());

                if user_options.reset_password {
                    println!("You asked to modify your password");

                    if let Err(e) = check_email_duration() {
                        println!("Error: {}", e);
                        return;
                    }

                    if let Err(e) = change_password(user.get_username()){
                        println!("Error: {}", e);
                        return;
                    }

                    println!("Password changed!");
                }

                if user_options.two_factors {
                    println!("You asked to modify your two_factors parameter");

                    match change_two_factors(user.get_username(), user.get_two_factors()) {
                        Ok(_) => println!("The usage of two factors has been changed!"),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
