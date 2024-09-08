pub mod user;
pub mod database;
pub mod schema;
pub mod templates;


// use rocket_sync_db_pools::database;
// use diesel::prelude::*;
// #[database("sqlite")]
// pub fn establish_connection() -> SqliteConnection {
//     SqliteConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }