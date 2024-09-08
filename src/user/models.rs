use chrono::NaiveDateTime;
use diesel::{prelude::*, Queryable, Insertable, AsChangeset};
use serde::{Deserialize, Serialize};
use crate::schema::users::{created_at, updated_at};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub is_view: bool,
    pub is_staff: bool,
    #[serde(skip_deserializing)]
    pub created_at: String,
    #[serde(skip_deserializing)]
    pub updated_at: String
}