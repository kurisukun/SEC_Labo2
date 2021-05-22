use crate::elements::user::User;
use crate::errors::Errors;
use rusqlite::Connection;

pub fn create_table() {
    let conn = establish_connection();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id integer not null primary key, 
        username varchar not null, 
        password varchar not null, 
        two_factors boolean not null, 
        secret varchar not null);",
        [],
    )
    .unwrap();
}

pub fn establish_connection() -> Connection {
    match Connection::open("db.sqlite") {
        Ok(conn) => conn,
        Err(e) => panic!("There was a problem opening the database: {:?}", e),
    }
}

pub fn user_exists(username: &str) -> Result<bool, Errors> {
    let conn = establish_connection();
    let result = conn.prepare("SELECT * FROM users WHERE username=?");

    match result {
        Ok(mut stmt) => Ok(stmt.exists(rusqlite::params![username]).unwrap()),
        Err(_) => Err(Errors::LoginError),
    }
}

pub fn get_user(username: &str) -> Result<User, Errors> {
    match user_exists(username) {
        Ok(exists) => {
            if exists {
                let conn = establish_connection();
                let result = conn.prepare("SELECT * FROM users WHERE username=?");

                match result {
                    Ok(mut stmt) => {
                        let mut rows = stmt.query([username]).unwrap();
                        let row = rows.next().unwrap().unwrap();
                        return Ok(User::new(
                            row.get(1).unwrap(),
                            row.get(2).unwrap(),
                            row.get(3).unwrap(),
                            row.get(4).unwrap(),
                        ));
                    }
                    Err(_) => return Err(Errors::GetUserError),
                }
            }
            Err(Errors::LoginError)
        }
        Err(_) => Err(Errors::LoginError),
    }
}

pub fn create_user(username: &str, password: &str) -> Result<(), Errors> {
    match user_exists(username) {
        Ok(exists) => {
            if !exists {
                let conn = establish_connection();
                let stmt = conn
                    .execute("INSERT INTO users (username, password, two_factors, secret) VALUES (?1, ?2, false, '')", [username, password]);

                match stmt {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err(Errors::CreateUserError),
                }
            }
            Err(Errors::EmailUsedError)
        }
        Err(_) => Err(Errors::GetUserError),
    }
}

pub fn update_user_secret(username: &str, two_factors: bool, secret: &str) -> Result<bool, Errors> {
    let conn = establish_connection();
    let mut two = '0';

    if two_factors {
        two = '1';
    }

    let stmt = conn.execute(
        "UPDATE users SET two_factors = $1, secret = $2 WHERE username = $3",
        &[two.to_string().as_str(), secret, username],
    );

    match stmt {
        Ok(_) => Ok(true),
        Err(e) => {
            println!("{}", e);
            Err(Errors::UpdateUserError)
        }
    }
}

pub fn update_user_password(username: &str, password: &str) -> Result<bool, Errors> {
    let conn = establish_connection();
    let stmt = conn.execute(
        "UPDATE users SET password = $1 WHERE username = $2",
        [password, username],
    );

    match stmt {
        Ok(_) => Ok(true),
        Err(e) => {
            println!("{}", e);
            Err(Errors::UpdateUserError)
        }
    }
}

#[cfg(test)]
mod test {

    fn drop_table() {
        let conn = establish_connection();
        conn.execute("DROP TABLE users", []).unwrap();
    }

    use super::*;
    #[test]
    fn valid_get_user() {
        let username = "test@test.com";
        let password = "1234";
        drop_table();
        create_table();
        let user_created = create_user(username, password).unwrap();
        assert_eq!(user_created, ());
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

    #[test]
    fn invalid_get_user() {
        drop_table();
        create_table();
        let username = "test@test.com";
        let error = get_user(username);
        println!("{:?}", error);
        assert_eq!(error.unwrap_err(), Errors::LoginError);
    }

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
