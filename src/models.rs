use super::schema::{users};

#[derive(Insertable, PartialEq, Debug, Default)]
#[table_name = "users"]
pub struct NewUser<'a> {
    username: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(username: &'a str) -> Self {
        Self {
            username
        }
    }
}
