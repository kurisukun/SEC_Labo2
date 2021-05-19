use rusqlite::{Connection};

pub fn connect_to_database() -> Connection{
    
    match Connection::open("db.sqlite"){
        Ok(connection) => connection,
        Err(e) => panic!("There was a problem opening the file: {:?}", e),
    }
}