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

mod schema;
mod models;
mod endpoints;
mod config;
mod env;
mod db;

mod endpoint_error;

use env::Env;
use config::DbConfig;
use db::Db;
use endpoints::api_v1;

fn main() {
    let env_str = &std_env::var("BLOG_ENV").unwrap_or(format!("development"));
    let env = Env::from_str(env_str).unwrap();
    let db_config = DbConfig::load(&env).expect("Error loading DB configuration");
    let mut db = Db::new(db_config);

    match db.init() {
        Ok(_) => {
            rocket::ignite().mount("/api/v1", routes![
                api_v1::posts::api_v1_posts_index,
                api_v1::posts::api_v1_posts_create,
                api_v1::posts::api_v1_posts_show,
                api_v1::posts::api_v1_posts_update,
                api_v1::posts::api_v1_posts_destroy,
                api_v1::posts::api_v1_users_posts_index,
                api_v1::users::api_v1_users_index,
                api_v1::users::api_v1_users_create,
                api_v1::users::api_v1_users_show,
                api_v1::users::api_v1_users_update,
                api_v1::users::api_v1_users_destroy,
                api_v1::comments::api_v1_comments_index,
                api_v1::comments::api_v1_comments_create,
                api_v1::comments::api_v1_comments_show,
                api_v1::comments::api_v1_comments_update,
                api_v1::comments::api_v1_comments_destroy
            ]).manage(db).launch()
        },
        Err(err) => println!("Db initialization error: {}", err)
    };
}
