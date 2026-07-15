use std::env;

use crate::{
    auth::dtos::{Login, Payload, Signup},
    connect_db,
    models::{User, UserSelect},
    schema::users::{self, dsl::*},
    shared::AppError,
};
use bcrypt::DEFAULT_COST;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub async fn login(payload: Login) -> Result<(String, Payload), AppError> {
    let mut conn = connect_db();
    let result = users
        .filter(email.eq(payload.email))
        .select(UserSelect::as_select())
        .first(&mut conn)
        .map_err(|err| match err {
            diesel::result::Error::NotFound => AppError::InvalidCredentials,
            other => AppError::Database(other),
        })?;
    let is_valid = bcrypt::verify(payload.password, &result.password)?;
    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }
    let payload = Payload {
        email: result.email,
        id: result.id,
        name: result.name,
    };
    let token = sign_jwt(&payload)?;
    Ok((token, payload))
}

pub async fn signup(payload: Signup) -> Result<(String, Payload), AppError> {
    let mut conn = connect_db();
    let hash_password = bcrypt::hash(&payload.password, DEFAULT_COST)?;

    let user = User {
        email: payload.email,
        name: payload.name,
        password: hash_password,
    };
    let result = diesel::insert_into(users::table)
        .values(&user)
        .returning(UserSelect::as_returning())
        .get_result(&mut conn)?;
    let payload = Payload {
        email: result.email,
        id: result.id,
        name: result.name,
    };
    let token = sign_jwt(&payload)?;
    Ok((token, payload))
}

fn sign_jwt(payload: &Payload) -> Result<String, AppError> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|err| AppError::InternalServerError)?;

    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|err| {
        //TODO: log error
        AppError::InternalServerError
    })?;
    Ok(token)
}
pub fn verify_jwt(token: String) -> Result<Payload, AppError> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|err| AppError::InternalServerError)?;
    let payload = decode::<Payload>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|err| {
        //TODO: log error
        AppError::InvalidToken
    })?
    .claims;
    Ok(payload)
}
