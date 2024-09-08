use diesel::SqliteConnection;
use rocket::{fs::FileServer, get, launch, routes};
use rocket_sync_db_pools::database;

extern crate iha_api_gateway;
#[get("/")]
fn index() -> iha_api_gateway::templates::Index {
    iha_api_gateway::templates::Index {
        title: "Index".to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
            iha_api_gateway::user::routes::get_users,
        ])
        .mount("/public", FileServer::from("public"))
        .attach(iha_api_gateway::database::sqlite::DbConn::fairing())
}
