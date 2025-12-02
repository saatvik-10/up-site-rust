use crate::db::Db;

impl Db {
    pub fn create_user(&self) {
        println!("Create User");
    }

    pub fn get_user(&self) -> String {
        String::from("1")
    }
}