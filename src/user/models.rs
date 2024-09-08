use chrono::NaiveDateTime;
#[derive(Queryable, Insertable, Deserialize, Debug, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub is_view: bool,
    pub is_staff: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}