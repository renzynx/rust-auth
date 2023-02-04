use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    sync::{Client, Collection},
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(val) => val.to_string(),
            Err(_) => format!("MONGO_URI not found in .env file"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust_mongo");
        let col: Collection<User> = db.collection("users");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            email: new_user.email,
            password: new_user.password,
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Failed to insert document.");

        Ok(user)
    }

    pub fn get_user(&self, email: String) -> Result<User, Error> {
        let user = self
            .col
            .find_one(Some(doc! {"email": email}), None)
            .ok()
            .expect("User not found");

        match user {
            Some(user) => Ok(user),
            None => Ok(User {
                id: None,
                email: "".to_string(),
                password: "".to_string(),
            }),
        }
    }
}
