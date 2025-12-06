use crate::{db::Db, schema::user};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl Db {
    pub fn user_sign_up(
        &mut self,
        username: String,
        password: String,
    ) -> Result<User, diesel::result::Error> {
        let user_id = Uuid::new_v4();

        let new_user = User {
            id: user_id.to_string(),
            username,
            password,
        };

        let new_user = diesel::insert_into(user::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(new_user)
    }

    pub fn user_sign_in(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<String, diesel::result::Error> {
        let user = user::table
            .filter(user::username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)
            .optional()?
            .ok_or(diesel::result::Error::NotFound)?;

        if user.password != input_password {
            return Ok(user.id);
        }

        Err(diesel::result::Error::NotFound)
    }
}
