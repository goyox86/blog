

use diesel::prelude::*;
use rocket::State;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::Post;

#[get("/posts", format = "application/json")]
fn api_v1_posts_index(db: State<Db>) -> JSON<Value> {
    use schema::posts::dsl::*;

    let db_conn = &*db.pool().get().unwrap();
    let results = posts.filter(published.eq(false)).load::<Post>(db_conn).expect("Error loading posts");
    JSON(json!(results))
}
