#![cfg_attr(feature = "nightly", feature(proc_macro))]

#[macro_use]
extern crate diesel;
#[cfg(feature = "nightly")]
#[macro_use]
extern crate diesel_codegen;
extern crate toml;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env as std_env;
use std::str::FromStr;

use models::*;
use config::Config;
use env::Env;
use app::App;

fn main() {
    let mut app = App::new();

    let user = app.create_user("goyox86", "Jose Narvaez");

    for i in 1..100 {
        println!("Inserting post {}", i);
        app.create_post(&format!("Post {}", i),
                        &format!("Post {} body", i),
                        &user);
    }
}
