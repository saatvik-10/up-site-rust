pub mod schema;

pub struct Db {
    conn: Connection
}

impl Default for Db {
    fn default() -> Self {
        
        Self { 
            conn
        }
    }
}

impl Db {
    pub fn create_user(&self) {
        println!("Create User");
    }

    pub fn create_website(&self) -> String {
        String::from("1")
    }
}
