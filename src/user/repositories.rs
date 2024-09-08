use diesel::prelude::*;
use crate::user::models::User;
use crate::schema::users;

pub struct UserRepository;
impl UserRepository {

    pub fn find_all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.order(users::id.desc()).limit(limit).load::<User>(c)
    }
}
