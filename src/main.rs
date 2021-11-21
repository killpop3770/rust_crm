#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};

#[get("/main")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}