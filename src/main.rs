use rocket::{catchers, fs::FileServer, get, launch, routes};

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
            iha_api_gateway::user::routes::get_user,
            iha_api_gateway::user::routes::register_user,
        ])
        .register("/", catchers![
            iha_api_gateway::catcher::not_found,
            iha_api_gateway::catcher::unprocessable_entity

        ])
        .mount("/public", FileServer::from("public"))
        .attach(iha_api_gateway::database::sqlite::DbConn::fairing())
}
