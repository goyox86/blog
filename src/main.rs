#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate toml;
extern crate r2d2;
extern crate r2d2_diesel;


use diesel::prelude::*;

use app::App;

pub mod schema;
pub mod app;
pub mod models;
pub mod config;
pub mod env;

fn main() {
    let mut app = App::new();
    app.start();
    app.db().get();
}
