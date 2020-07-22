use serde::Serialize;

use diesel::prelude::*;

use crate::schema::users;
use crate::schema::users::dsl::*;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub username: String,
    // ! Password should be hashed
    pub password: String,
}

impl User {
    pub fn insert(&self, connection: &PgConnection) -> Result<User, diesel::result::Error> {
        let new_user = User {
            username: self.username.to_owned(),
            password: self.password.to_owned(),
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(connection)?;

        Ok(new_user)
    }

    pub fn get(&self, connection: &PgConnection) -> Result<Option<User>, diesel::result::Error> {
        let user = users
            .filter(username.eq(self.username.to_owned()))
            .filter(password.eq(self.password.to_owned()))
            .first::<User>(connection)
            .optional()?;

        Ok(user)
    }
}
