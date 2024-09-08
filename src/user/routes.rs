// #[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Value};
use rocket_sync_db_pools::Connection;
use rocket_sync_db_pools::database;

use crate::database::sqlite::{DbConn};
use crate::schema::users;
use crate::user::models::User;
use crate::user::repositories::UserRepository;


#[get("/users")]
pub async fn get_users(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        UserRepository::find_all(c, 100)
            .map(|users| json!(users))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}