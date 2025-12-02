use crate::config::Config;
use diesel::{Connection, ConnectionError, PgConnection};

pub struct Db {
    pub conn: PgConnection,
}

impl Db {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();

        let conn = PgConnection::establish(&config.db_url)?;

        Ok(Self { conn })
    }
}
