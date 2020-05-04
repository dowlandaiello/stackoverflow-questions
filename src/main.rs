#[macro_use]
extern crate diesel;

use diesel::{
    mysql::{Mysql, MysqlConnection},
    sql_types::{Bool, Nullable},
    BoxableExpression, Connection, ExpressionMethods, RunQueryDsl,
};
use schema::{roles, users};
use models::{NewUser};

use std::error::Error;

mod schema {
    table! {
        roles (user_id) {
            id -> Unsigned<Bigint>,
            user_id -> Unsigned<Bigint>,
            administrator -> Nullable<Bool>,
            sponsor -> Nullable<Bool>,
            bot -> Nullable<Bool>,
        }
    }

    table! {
        users (id) {
            id -> Unsigned<Bigint>,
            username -> Nullable<Varchar>,
        }
    }

    allow_tables_to_appear_in_same_query!(
        roles,
        users,
    );
}

mod models {
    use super::schema::users;

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
}

enum Role {
    Administrator,
    Sponsor,
    Bot,
}

impl From<&Role> for Box<dyn BoxableExpression<roles::table, Mysql, SqlType = Nullable<Bool>>> {
    fn from(r: &Role) -> Self {
        match r {
            Role::Administrator => Box::new(roles::dsl::administrator),
            Role::Sponsor => Box::new(roles::dsl::sponsor),
            Role::Bot => Box::new(roles::dsl::bot),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = MysqlConnection::establish("mysql://localhost/stquestion")?;

    diesel::replace_into(users::table)
        .values(&NewUser::new("test_account"))
        .execute(&conn)?;
    diesel::replace_into(roles::table)
        .values((
            roles::dsl::user_id.eq(1),
            <&Role as Into<
                Box<dyn BoxableExpression<roles::table, Mysql, SqlType = Nullable<Bool>>>,
            >>::into(&Role::Administrator)
            .eq(true),
        ))
        .execute(&conn)?;

    Ok(())
}
