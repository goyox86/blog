#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate toml;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env as std_env;
use std::str::FromStr;

pub mod schema;
pub mod models;
pub mod endpoints;
pub mod config;
pub mod env;
pub mod db;

use env::Env;
use config::DbConfig;
use db::Db;
use endpoints::api_v1;

fn main() {
    let env_str = &std_env::var("BLOG_ENV").unwrap_or(format!("development"));
    let env = Env::from_str(env_str).unwrap();
    let db_config = DbConfig::load(&env).expect("Error loading DB configuration");
    let mut db = Db::new(db_config);
    db.init();

    let api_v1_routes = routes![api_v1::posts::api_v1_posts_index,
                                api_v1::posts::api_v1_posts_create,
                                api_v1::posts::api_v1_posts_show,
                                api_v1::posts::api_v1_posts_update,
                                api_v1::posts::api_v1_posts_destroy];

    rocket::ignite()
        .mount("/api/v1", api_v1_routes)
        .manage(db)
        .launch()
}
