use diesel::prelude::*;
use crate::user::models::{NewUser, User};
use crate::schema::users;

pub struct UserRepository;
impl UserRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(c)
    }

    pub fn find_all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.order(users::id.desc()).limit(limit).load::<User>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(c)?;

        let last_id = Self::last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    // pub fn update(c: &mut SqliteConnection, id: i32, user: User) -> QueryResult<User> {
    //     diesel::update(users::table.find(id))
    //         .set((
    //             users::username.eq(user.username.to_owned()),
    //             users::email.eq(user.email.to_owned())
    //         ))
    //         .execute(c)?;
    //
    //     Self::find(c, id)
    // }
    //
    // pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
    //     diesel::delete(users::table.find(id)).execute(c)
    // }


    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        users::table.select(users::id).order(users::id.desc()).first(c)
    }

}
