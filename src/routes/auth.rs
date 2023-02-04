use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, response::content, serde::json::Json, State};

#[post("/register", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let password = new_user.password.to_owned();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let data = User {
        id: None,
        email: new_user.email.to_owned(),
        password: hash,
    };

    let user = db.create_user(data);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data = "<user>")]
pub fn login_user(
    db: &State<MongoRepo>,
    user: Json<User>,
) -> Result<content::RawJson<&'static str>, Status> {
    let doc = db.get_user(user.email.to_owned());

    match doc {
        Ok(doc) => {
            if !doc.email.is_empty() {
                let argon2 = Argon2::default();
                let parsed_hash = PasswordHash::new(&doc.password).unwrap();
                let valid = argon2
                    .verify_password(user.password.as_bytes(), &parsed_hash)
                    .is_ok();

                if valid {
                    Ok(content::RawJson(r#"{"message": "Login successful"}"#))
                } else {
                    Err(Status::Unauthorized)
                }
            } else {
                return Err(Status::Unauthorized);
            }
        }
        Err(err) => {
            println!("{:?}", err);
            Err(Status::InternalServerError)
        }
    }
}
