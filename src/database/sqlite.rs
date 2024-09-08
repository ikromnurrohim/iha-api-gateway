use rocket_sync_db_pools::database;
use diesel::prelude::*;
#[database("sqlite")]
pub struct DbConn(SqliteConnection); // its just a pool
