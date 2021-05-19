mod db;
mod validation;
mod authentication;

use structopt::StructOpt;
use db::database::Database;
use validation::validation::{syntatic_validation_password, syntatic_validation_username};
use authentication::register::register;

#[derive(Debug)]
struct User {
    id: i32,
    email: String,
    password: String,
    two_factors: bool,
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
    #[structopt(long = "username", default_value = "")]
    username: String,
    //arg for user password
    #[structopt(long = "password", default_value = "")]
    password: String,
    //arg to register
    #[structopt(
        short,
        long,
        help = "Argument to enable/disable the two factors authentication"
    )]
    twofactors: bool,
}

fn main() {
    let database = Database::new("db.sqlite");
    let conn = database.get_conn();
    let mut stmt = conn.prepare("SELECT * FROM users").unwrap();
    let person_iter = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                two_factors: row.get(3)?,
            })
        })
        .unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    let user_options = Options::from_args();
    println!("{:?}", user_options);

    if !syntatic_validation_username(&user_options.username) && !syntatic_validation_password(&user_options.password) {
        println!("Error: credentials not valid!");
        return; 
    }

    if user_options.register{
        register();
    }
    else{

    }

}
