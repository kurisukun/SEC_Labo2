use rusqlite::{Connection, Error};
use crate::elements::user::User;

pub fn create_table() {
    let conn = establish_connection();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id integer not null primary key, 
        username varchar not null, 
        password varchar not null, 
        two_factors boolean not null, 
        google_token varchar not null);",
        [],
    )
    .unwrap();
}

pub fn drop_table() {
    let conn = establish_connection();
    conn.execute("DROP TABLE users", []).unwrap();
}

pub fn establish_connection() -> Connection {
    match Connection::open("db.sqlite") {
        Ok(conn) => conn,
        Err(e) => panic!("There was a problem opening the database: {:?}", e),
    }
}

pub fn user_exists(username: &str) -> Result<bool, rusqlite::Error> {
    let conn = establish_connection();
    let result = conn.prepare("SELECT * FROM users WHERE username=?");

    match result {
        Ok(mut stmt) => stmt.exists(rusqlite::params![username]),
        Err(e) => Err(e),
    }
}

pub fn get_user(username: &str) -> Result<User, Error> {
    match user_exists(username) {
        Ok(_) => {
            let conn = establish_connection();
            let result = conn.prepare("SELECT * FROM users WHERE username=?");

            match result {
                Ok(mut stmt) => {
                    let mut rows = stmt.query([username]).unwrap();
                    let row = rows.next().unwrap().unwrap();
                    Ok(User::new(
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                    ))
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn create_user(username: &str, password: &str) -> Result<bool, Error> {
    match user_exists(username) {
        Ok(exists) => {
            if !exists {
                let conn = establish_connection();
                let stmt = conn
                    .execute("INSERT INTO users (username, password, two_factors, google_token) VALUES (?1, ?2, false, '')", [username, password]);

                match stmt {
                    Ok(_) => return Ok(true),
                    Err(e) => return Err(e),
                };
            }

            println!("User already exists!");
            Ok(false)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn valid_get_user() {
        let username = "test@test.com";
        let password = "1234";
        drop_table();
        create_table();
        let user_created = create_user(username, password).unwrap();
        assert!(user_created);
        let user_test = get_user(username).unwrap();
        assert_eq!(
            User::new(
                username.to_string(),
                password.to_string(),
                false,
                "".to_string()
            ),
            user_test
        );
    }

    /*
    #[test]
    fn invalid_get_user(){
        let username = "test@test.com";
        drop_table();
        create_table();
        let error = get_user(username);
        println!("{:?}", error);
        //assert_eq!(error.unwrap_err(), Error::QueryReturnedNoRows);
        assert!(true);
    }*/

    #[test]
    fn valid_user_exists() {
        drop_table();
        create_table();
        let username = "test@test.com";
        let password = "1234";
        let _user_created = create_user(username, password).unwrap();
        let exists = user_exists(username).unwrap();
        assert!(exists);
    }

    #[test]
    fn invalid_user_exists() {
        drop_table();
        create_table();
        let username = "test@test.com";
        let exists = user_exists(username).unwrap();
        assert!(!exists);
    }
}
