use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    admin::dtos::Update_dto,
    connect_db,
    models::{User, UserSelect},
    schema::users::{self as usersTable, dsl::*},
    shared::{self, AppError},
};

pub async fn get_users() -> Result<Vec<UserSelect>, AppError> {
    let mut conn = connect_db();
    let result = users.select(UserSelect::as_select()).load(&mut conn)?;

    Ok(result)
}

pub async fn update_user(user_id:Uuid,payload: Update_dto) -> Result<User, AppError> {
    let mut conn = connect_db();
    let updated_user= diesel::update(users.filter(id.eq(user_id)))
        .set(&payload)
        .returning(User::as_returning())
        .get_result(&mut conn)?;
    Ok(updated_user)
}
pub async fn delete_user(user_id:Uuid) -> Result<String, AppError> {
    let mut conn = connect_db();

    diesel::delete(users.filter(id.eq(user_id))).execute(&mut conn)?;
    Ok("User deleted".to_string())
}

pub async fn get_user_by_id(user_id: Uuid) -> Result<UserSelect, shared::AppError> {
    let mut conn = connect_db();
    let user = usersTable::table
        .filter(usersTable::id.eq(user_id))
        .get_result(&mut conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => AppError::NotFound,
            other => other.into(),
        })?;
    Ok(user)
}
