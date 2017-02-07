#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate toml;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use diesel::prelude::*;
use rocket::State;
use rocket_contrib::{JSON, Value};
use std::env as std_env;
use std::str::FromStr;

pub mod schema;
pub mod models;
pub mod config;
pub mod env;
pub mod db;

use models::Post;
use env::Env;
use config::DbConfig;
use db::Db;

fn main() {
    let env_str = &std_env::var("BLOG_ENV").unwrap_or(format!("development"));
    let env = Env::from_str(env_str).unwrap();
    let db_config = DbConfig::load(&env).expect("Error loading DB configuration");
    let mut db = Db::new(db_config);
    db.init();

    rocket::ignite()
        .mount("/", routes![posts_index])
        .manage(db)
        .launch()
}

#[get("/", format = "application/json")]
fn posts_index(db: State<Db>) -> JSON<Value> {
    use schema::posts::dsl::*;

    let db_conn = &*db.pool().get().unwrap();
    let results = posts.filter(published.eq(false)).load::<Post>(db_conn).expect("Error loading posts");
    JSON(json!(results))
}
