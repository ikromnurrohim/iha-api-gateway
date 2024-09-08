use rocket::{post, get};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Value, json, Json};

use crate::database::sqlite::{DbConn};
use crate::user::models::{NewUser};
use crate::user::repositories::UserRepository;


#[get("/users")]
pub async fn get_users(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        UserRepository::find_all(c, 100)
            .map(|users| json!(users))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/user/<id>")]
pub async fn get_user(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move|c| {
        UserRepository::find(c, id)
            .map(|users| json!(users))
            .map_err(|e|Custom(Status::NotFound, json!(e.to_string())))
    }).await
}

#[post("/user/register", format="json", data = "<new_user>")]
pub async fn register_user(db: DbConn, new_user: Json<NewUser>) -> Result<Value, Custom<Value>> {
    println!("{:?}", new_user);
    db.run(|c| {
        UserRepository::create(c, new_user.into_inner())
            .map(|user| json!(user))
            .map_err(|e|Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}