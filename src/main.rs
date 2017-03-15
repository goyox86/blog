#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(needless_pass_by_value))]

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
extern crate bcrypt;

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
use endpoints::*;

fn main() {
    let env_str = &std_env::var("BLOG_ENV").unwrap_or_else(|_| "development".to_owned());
    let env = Env::from_str(env_str).unwrap_or_default();
    let db_config = DbConfig::load(&env).expect("Error loading DB configuration");
    let mut db = Db::new(db_config);

    match db.init() {
        Ok(_) => {
            rocket::ignite().mount("/api/v1", routes![
                api_v1::posts::index,
                api_v1::posts::index_paginated,
                api_v1::posts::create,
                api_v1::posts::show,
                api_v1::posts::update,
                api_v1::posts::destroy,
                api_v1::posts::user_posts_index,
                api_v1::posts::user_post_show,
                api_v1::users::index,
                api_v1::users::index_paginated,
                api_v1::users::create,
                api_v1::users::show,
                api_v1::users::update,
                api_v1::users::destroy,
                api_v1::comments::index,
                api_v1::comments::index_paginated,
                api_v1::comments::create,
                api_v1::comments::show,
                api_v1::comments::update,
                api_v1::comments::destroy,
                api_v1::comments::post_comments_index,
                api_v1::comments::user_comments_index,
                api_v1::comments::post_comment_show,
            ]).mount("/auth", routes![
                auth::basic::login,
                auth::basic::login_json,
            ]).manage(db)
            .launch()
        }
        Err(err) => println!("Db initialization error: {}", err),
    };
}
