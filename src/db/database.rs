use rusqlite::Connection;

pub struct Database{
    conn: Connection,
}


impl Database {
    
    pub fn new(db: &str)-> Self{
        match Connection::open("db.sqlite") {
            Ok(conn) => Self{conn},
            Err(e) => panic!("There was a problem opening the database: {:?}", e),
        }
    }

    pub fn get_conn(self) -> Connection{
        self.conn
    }
}

