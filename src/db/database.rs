#[path = "../elements/user.rs"]
pub mod user;
use rusqlite::{Connection, Error};

use self::user::User;



 pub fn establish_connection() -> Connection{
    match Connection::open("db.sqlite") {
        Ok(conn) => conn,
        Err(e) => panic!("There was a problem opening the database: {:?}", e),
    }
 }


pub fn user_exists(username: &str) -> Result<bool, rusqlite::Error>{

    let conn = establish_connection();
    let mut stmt = conn
        .prepare("SELECT * FROM users WHERE username=?")
        .unwrap();

    stmt.exists(rusqlite::params![username])
}

pub fn get_user(username: &str) -> Result<User, Error>{
    match user_exists(username){
        Ok(_) => {
            let conn = establish_connection();
            let mut stmt = conn
                .prepare("SELECT * FROM users WHERE username=?")
                .unwrap();
            
            let mut rows = stmt.query([username]).unwrap();
            let row = rows.next().unwrap().unwrap();
            Ok(User::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }, 
        Err(e) => Err(e),
    }
}


pub fn create_user(username: &str, password: &str) -> Result<(), Error>{

    match user_exists(username){
        Err(e) => {
            Err(e)
        }
        Ok(exists) => {
            println!("Exists: {}", exists);
            if !exists{
                let conn = establish_connection();
                let mut stmt = conn
                    .execute("INSERT INTO users (username, password, two_factors) VALUES (?1, ?2, false)", [username, password])
                    .unwrap();
                
                return Ok(())
            }

            println!("User already exists!");
            Err(Error::ExecuteReturnedResults)
        },
    }
}