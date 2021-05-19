
use rusqlite::{Connection, Result};

mod db;

#[derive(Debug)]
struct User {
    id: i32,
    email: String,
    password: String,
    two_factors: bool,
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
}
