#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate toml;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env as std_env;
use std::str::FromStr;

use app::App;

pub mod schema;
pub mod app;
pub mod models;
pub mod config;
pub mod env;

fn main() {
    let mut app = App::new();
    app.start();

    let user = app.create_user("goyox86", "Jose Narvaez");

    for i in 1..100 {
        println!("Inserting post {}", i);
        app.create_post(&format!("Post {}", i), &format!("Post {} body", i), &user);
    }
}
