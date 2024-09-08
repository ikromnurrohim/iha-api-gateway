use rocket::catch;
use rocket::serde::json::{Value, json};
#[catch(404)]
pub fn not_found() -> Value {
    json!("Not found!")
}
#[catch(422)]
pub fn unprocessable_entity() -> Value {
    json!("Unprocessable Entity!")
}
