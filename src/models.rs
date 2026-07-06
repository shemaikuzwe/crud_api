use crate::schema::users;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Insertable,Serialize)]
#[diesel(table_name=users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Queryable, Selectable, Insertable,Serialize)]
#[diesel(table_name=users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSelect {
    //fields should be correctly arranged as defined in schema
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}
