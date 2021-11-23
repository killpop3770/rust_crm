use dotenv::dotenv;
// use rocket::Config;
use std::env;
use std::collections::HashMap;
use rocket::figment::value::{Map, Value};
use rocket::figment::map;
use rocket::figment::Figment;
use rocket::outcome::Outcome;

pub fn from_env() -> Figment {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let db: Map<_, Value> = map! {
        "url" => database_url.into(),
        "pool_size" => 10.into()
    };
    let figment = rocket::Config::figment().merge(("databases", map!["rust_crm" => db]));

    return figment;
}
