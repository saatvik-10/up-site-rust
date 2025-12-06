use std::sync::{Arc, Mutex};

use crate::states::{JwtOutput, UserInput, UserOutput};
use db::db::Db;
use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json},
};

#[handler]
pub fn user_sign_up(
    Json(data): Json<UserInput>,
    Data(db): Data<&Arc<Mutex<Db>>>,
) -> Result<Json<UserOutput>, Error> {
    let mut db_lock = db.lock().unwrap();
    let user = db_lock
        .user_sign_up(data.username, data.password)
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    let res = UserOutput {
        id: user.id,
        username: user.username,
    };

    Ok(Json(res))
}

#[handler]
pub fn user_sign_in(
    Json(data): Json<UserInput>,
    Data(db): Data<&Arc<Mutex<Db>>>,
) -> Result<Json<JwtOutput>, Error> {
    let mut db_lock = db.lock().unwrap();
    let user_id = db_lock.user_sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let res = JwtOutput { jwt: user_id };
            Ok(Json(res))
        }
        Err(_e) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}
