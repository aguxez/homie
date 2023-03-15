use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct Pool {
    pub conn: SqliteConnection,
}

impl Pool {
    pub fn new(url_or_path: String) -> Pool {
        let conn = SqliteConnection::establish(&url_or_path)
            .unwrap_or_else(|_| panic!("Error connecting to {}", url_or_path));

        Pool { conn }
    }
}
