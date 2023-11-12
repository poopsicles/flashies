use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::NewFile;

pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_file(conn: &mut MysqlConnection, name: &str, hash: &str, data: &[u8]) -> Result<usize, diesel::result::Error> {
    use crate::schema::files;

    let f = NewFile { name, hash, data };

    diesel::insert_into(files::table)
        .values(&f)
        .execute(conn)
}
