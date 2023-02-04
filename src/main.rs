mod models;
mod repository;
mod routes;

#[macro_use]
extern crate rocket;

use repository::mongodb_repo::MongoRepo;
use routes::auth::{create_user, login_user};

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![login_user])
}
