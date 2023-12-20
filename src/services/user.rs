use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::establish_connection_pg;

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[post("/user", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUser>) -> Result<String, String> {
    use crate::models::User;
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection_pg();

    let users_with_same_email_or_name = users
        .or_filter(
            email
                .eq(user.email.to_string())
                .or(username.eq(user.name.to_string())),
        )
        .limit(1)
        .load::<User>(connection)
        .expect("Failed to load users");

    if users_with_same_email_or_name.len() > 0 {
        println!("User with the name or email already exists");
        return Err(String::from("User with the name or email already exists"));
    }

    let user_password = user.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(user_password, &salt)
        .expect("Failed to hash password")
        .to_string();

    let new_user = User {
        uuid: Uuid::new_v4(),
        username: user.name.to_string(),
        email: user.email.to_string(),
        password: password_hash,
    };

    diesel::insert_into(users)
        .values(new_user)
        .execute(connection)
        .expect("Error saving new user");

    return Ok(String::from("Success"));
}
