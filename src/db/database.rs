use crate::elements::user::User;
use crate::errors::Errors;
use rusqlite::Connection;

const DB_NAME : &str = "db.sqlite";

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
    match Connection::open(DB_NAME) {
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
        Ok(size) => {
            if size == 0{
                return Err(Errors::UpdateUserError);
            }
            Ok(true)
        },
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
        Ok(size) => {
            if size == 0{
                return Err(Errors::UpdateUserError);
            }
            Ok(true)
        },
        Err(e) => {
            println!("{}", e);
            Err(Errors::UpdateUserError)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn drop_table() {
        let conn = establish_connection();
        conn.execute("DROP TABLE users", []).unwrap();
    }

    fn create_user_for_tests(username: &str, password: &str){
        drop_table();
        create_table();
        let _result = create_user(username, password);
    }

    #[test]
    fn valid_get_user() {
        let username = "test@test.com";
        let password = "1234";
        create_user_for_tests(username, password);
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
        let username = "test@test.com";
        let password = "1234";
        create_user_for_tests(username, password);
        let exists = user_exists(username).unwrap();
        assert_eq!(exists, true);
    }

    #[test]
    fn invalid_user_exists() {
        drop_table();
        create_table();
        let username = "test@test.com";
        let exists = user_exists(username).unwrap();
        assert_eq!(exists, false);
    }
    #[test]
    fn valid_create_user(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let password = "1234";
        let user_created_result = create_user(username, password).unwrap();
        assert_eq!(user_created_result, ());
        let user = get_user(username).unwrap();
        assert_eq!(user, User::new(username.to_string(), password.to_string(), false, "".to_string()));
    }

    #[test]
    fn invalid_create_user(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let password = "1234";
        let _user_created_result1 = create_user(username, password).unwrap();
        //Won't pass since user already exists
        let user_created_result2 = create_user(username, password).unwrap_err();
        assert_eq!(user_created_result2, Errors::EmailUsedError);
    }

    //TODO test update_user_secret
    #[test]
    fn valid_update_user_secret(){
        let username = "test@test.com";
        let password = "1234";
        create_user_for_tests(username, password);
        let secret = "MySecret";
        let two_factors = true;
        let result = update_user_secret(username, two_factors, secret).unwrap();
        assert_eq!(result, true);
        let user = get_user(username).unwrap();
        assert_eq!(user, User::new(username.to_string(), password.to_string(), two_factors, secret.to_string()));
    }

    #[test]
    fn invalid_update_user_secret(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let secret = "MySecret";
        let two_factors = true;
        let _user = get_user(username);
        let result = update_user_secret(username, two_factors, secret).unwrap_err();
        assert_eq!(result, Errors::UpdateUserError);
    }

    //TODO test update_user_password
    #[test]
    fn valid_update_user_password(){
        let username = "test@test.com";
        let password = "1234";
        create_user_for_tests(username, password);
        //Such wow, much security
        let new_password = "12345";
        let result = update_user_password(username, new_password).unwrap();
        assert_eq!(result, true);
        let user = get_user(username).unwrap();
        assert_eq!(user, User::new(username.to_string(), new_password.to_string(), false, "".to_string()));
    }

    #[test]
    fn invalid_update_user_password(){
        drop_table();
        create_table();
        let username = "test@test.com";
        let new_password = "12345";
        let _user = get_user(username);
        let result = update_user_password(username, new_password).unwrap_err();
        assert_eq!(result, Errors::UpdateUserError);
    }
}
