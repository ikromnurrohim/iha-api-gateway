use rocket_sync_db_pools::database;
use diesel::prelude::*;
#[database("sqlite")]
pub struct DbConn(SqliteConnection); // its just a pool




// #[database("sqlite")]
// pub fn establish_connection() -> SqliteConnection {
//     SqliteConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }
