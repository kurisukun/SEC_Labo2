use structopt::StructOpt;

mod db;
mod validation;

#[derive(Debug)]
struct User {
    id: i32,
    email: String,
    password: String,
    two_factors: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sec_labo2", about = "SEC Labo2: two-factor authentication program using Google Authenticator")]
struct Options{
    
    //arg to login
    #[structopt(short, long, help = "Argument to proceed a login")]
    login: bool,
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
    #[structopt(short, long, help = "Argument to enable/disable the two factors authentication")]
    twofactors: bool,
}

fn main(){
    let conn = db::database::connect_to_database();
    let mut stmt = conn.prepare("SELECT * FROM users").unwrap();
    let person_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
            two_factors: row.get(3)?,
        })
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    let opt = Options::from_args();
    println!("{:?}", opt);
}
