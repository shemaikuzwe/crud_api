use crate::{
    auth::dtos::{AuthError, Login, Signup},
    connect_db,
    models::User,
    schema::users::{self, dsl::*},
};
use bcrypt::DEFAULT_COST;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub async fn login(payload: Login) -> Result<String, AuthError> {
    let mut conn = connect_db();
    let result = users
        .filter(email.eq(payload.email))
        .select(User::as_select())
        .first(&mut conn)?;
    let is_valid = bcrypt::verify(payload.password, &result.password)?;
    if !is_valid {
        return Err(AuthError::InvalidCredentials);
    }
    Ok("user logged in successfully".to_string())
}

pub async fn signup(payload: Signup) -> Result<String, AuthError> {
    let mut conn = connect_db();
    let hash_password = bcrypt::hash(&payload.password, DEFAULT_COST)?;

    let user = User {
        email: payload.email,
        name: payload.name,
        password: hash_password,
    };
    diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(&mut conn)?;
    
    Ok("user created succesffuly".to_string())
}
