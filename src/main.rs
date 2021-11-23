#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket_contrib::databases::database;
use rocket_contrib::databases::postgres;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

mod config;

#[database("diesel_postgres_pool")]
pub struct DBPool(postgres::Connection);


#[get("/main")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::custom(config::from_env())
        .mount("/", routes![index])
        .attach(DBPool::fairing())
}