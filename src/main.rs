#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[cfg(feature = "nightly")]
#[macro_use]
extern crate diesel_codegen;
extern crate toml;
extern crate rocket;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env as std_env;
use std::str::FromStr;

use app::App;

fn main() {
    let mut app = App::new();
    app.init();
    &app.start();
}
