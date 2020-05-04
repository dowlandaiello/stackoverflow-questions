use diesel::{
    mysql::{Mysql, MysqlConnection},
    sql_types::{Bool, Nullable},
    BoxableExpression, Connection, ExpressionMethods, RunQueryDsl,
};
use stackoverflow_questions::{
    models::NewUser,
    schema::{roles, users},
};

use std::error::Error;

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
