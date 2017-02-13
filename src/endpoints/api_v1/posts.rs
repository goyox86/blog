

use diesel::prelude::*;
use rocket::State;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::Post;
use schema::posts::dsl::*;

#[get("/posts", format = "application/json")]
fn api_v1_posts_index(db: State<Db>) -> JSON<Value> {
    let db_conn = &*db.pool().get().unwrap();
    let results = posts.filter(published.eq(false)).load::<Post>(db_conn).expect("Error loading posts");
    JSON(json!(results))
}

#[post("/posts", format = "application/json")]
fn api_v1_posts_create(db: State<Db>) -> JSON<Value> {
    unimplemented!()
}

#[get("/posts/<post_id>", format = "application/json")]
fn api_v1_posts_show(post_id: i32, db: State<Db>) -> JSON<Value> {
    unimplemented!()
}

#[put("/posts/<post_id>", format = "application/json")]
fn api_v1_posts_update(post_id: i32, db: State<Db>) -> JSON<Value> {
    unimplemented!()
}

#[delete("/posts/<post_id>", format = "application/json")]
fn api_v1_posts_destroy(post_id: i32, db: State<Db>) -> JSON<Value> {
    unimplemented!()
}

