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
use rocket::Rocket;
use rocket_contrib::{JSON, Value};

use app::App;
use models::Post;

pub mod schema;
pub mod app;
pub mod models;
pub mod config;
pub mod env;

fn main() {
    let mut app = App::new();
    app.start();

    rocket::ignite()
        .mount("/", routes![posts_index])
        .manage(app)
        .launch()
}

#[get("/", format = "application/json")]
fn posts_index(app: State<App>) -> JSON<Value> {
    use schema::posts::dsl::*;

    let results = posts.filter(published.eq(false)).load::<Post>(&*app.db().get().unwrap()).expect("Error loading posts");
    JSON(json!(results))
}
