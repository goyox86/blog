#![cfg_attr(feature = "nightly", feature(proc_macro))]

#[macro_use] extern crate diesel;
#[cfg(feature = "nightly")]
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate toml;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use models::*;
use config::DatabaseConfig;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();


    let db_config = DatabaseConfig::load().expect("Ooops");
    let database_url = db_config.url();
    println!("{}", database_url);
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_user<'a>(conn: &PgConnection, username: &'a str,  name: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        username: username,
        name: name,
    };

    diesel::insert(&new_user).into(users::table)
        .get_result::<User>(conn)
        .expect("Error saving new user")
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str,  body: &'a str, user: &User) {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
        user_id: Some(user.id)
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result::<Post>(conn)
        .expect("Error saving new post");
}

fn main() {
    use schema::posts::dsl::*;
    let conn = establish_connection();

    let user = create_user(&conn, "goyox86", "Jose Narvaez");

    for i in 1..100 {
        println!("Inserting post {}", i);
        create_post(&conn, &format!("Post {}", i), &format!("Post {} body", i), &user);
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
