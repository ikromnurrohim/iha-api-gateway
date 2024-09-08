use rocket::{fs::FileServer, get, launch, routes};

mod templates;
mod database;
use database::{DbConn};



#[get("/")]
fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
        .attach(DbConn::fairing())
}
