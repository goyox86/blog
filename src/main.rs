#![cfg_attr(feature = "nightly", feature(proc_macro))]

#[macro_use] extern crate diesel;
#[cfg(feature = "nightly")]
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str,  body: &'a str) {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result::<Post>(conn)
        .expect("Error saving new post");
}

fn main() {
    use schema::posts::dsl::*;
    let conn = establish_connection();

    for i in 1..100 {
        println!("Inserting post {}", i);
        create_post(&conn, &format!("Post {}", i), &format!("Post {} body", i));
    }

    let results = posts.filter(published.eq(false))
        .load::<Post>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("------------\n");
        println!("{}", post.body);
    }
}
