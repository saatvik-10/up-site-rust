use crate::{db::Db, schema::website};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub time_added: chrono::NaiveDateTime,
}

impl Db {
    pub fn create_website(
        &mut self,
        user_id: String,
        url: String,
    ) -> Result<Website, diesel::result::Error> {
        let website_id = Uuid::new_v4();

        let new_website = Website {
            id: website_id.to_string(),
            url,
            user_id,
            time_added: Utc::now().naive_local(),
        };

        let new_website = diesel::insert_into(website::table)
            .values(&new_website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;

        Ok(new_website)
    }

    pub fn get_website(
        &mut self,
        input_website_id: String,
    ) -> Result<Website, diesel::result::Error> {
        let website = website::table
            .filter(website::id.eq(input_website_id))
            .select(Website::as_select())
            .first(&mut self.conn)
            .optional()?
            .ok_or(diesel::result::Error::NotFound)?;

        Ok(website)
    }
}
