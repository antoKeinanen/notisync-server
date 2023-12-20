extern crate rocket;

use rocket::{launch, routes};

mod models;
mod schema;
mod services;

use services::user;

#[launch]
fn rocker() -> _ {
    rocket::build().mount("/", routes![user::create_user])
}
