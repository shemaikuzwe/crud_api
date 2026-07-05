use crate::schema::users;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable,Insertable)]
#[diesel(table_name=users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}
