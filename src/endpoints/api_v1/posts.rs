use std::error::Error;
use std::io::Cursor;

use diesel::prelude::*;
use diesel;
use rocket::{State, Request, Response, Error as RocketError};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::Post;
use models::NewPost;
use schema::posts::dsl::*;
use schema::posts;

#[get("/posts", format = "application/json")]
fn api_v1_posts_index(db: State<Db>) -> JSON<Value> {
    let conn = &*db.pool().get().unwrap();
    let results =
        posts.filter(published.eq(false)).load::<Post>(conn).expect("Error loading posts");
    JSON(json!(results))
}

#[post("/posts", data = "<new_post>", format = "application/json")]
fn api_v1_posts_create(db: State<Db>,
                       new_post: JSON<NewPost>)
                       -> Result<JSON<Post>, diesel::result::Error> {
    //FIXME: Remove this unwrap
    let conn = &*db.pool().get().unwrap();

    diesel::insert(&new_post.0)
        .into(posts::table)
        .get_result::<Post>(conn)
        .map(|post| JSON(post))
        .map_err(|err| err)
}

#[get("/posts/<post_id>", format = "application/json")]
fn api_v1_posts_show(post_id: i32, db: State<Db>) -> Result<JSON<Post>, diesel::result::Error> {
    //FIXME: Remove this unwrap
    let conn = &*db.pool().get().unwrap();

    posts.find(post_id)
        .first(conn)
        .map(|post| JSON(post))
        .map_err(|err| err)
}

#[put("/posts/<post_id>", format = "application/json")]
fn api_v1_posts_update(post_id: i32, db: State<Db>) -> JSON<Value> {
    unimplemented!()
}

#[delete("/posts/<post_id>", format = "application/json")]
fn api_v1_posts_destroy(post_id: i32, db: State<Db>) -> Result<Response, diesel::result::Error> {
    //FIXME: Remove this unwrap
    let conn = &*db.pool().get().unwrap();

    match diesel::delete(posts.find(post_id)).get_result::<Post>(conn) {
        // TODO check why when I do Ok(json_response_with_status(Status::NoContent, json!({"status": "not_content"})))
        // it fails miserably
        Ok(_) => Ok(empty_response_with_status(Status::NoContent)),
        Err(err) => {
            match err {
                diesel::result::Error::NotFound => {
                    Ok(json_response_with_status(Status::NotFound, json!({"status": "not_found"})))
                }
                _ => Err(err),
            }
        }
    }
}

fn empty_response_with_status<'r>(status: Status) -> Response<'r> {
    let mut response = Response::new();
    response.set_status(status);
    response
}

fn json_response_with_status<'r>(status: Status, json: Value) -> Response<'r> {
    let mut response = empty_response_with_status(status);
    response.set_sized_body(Cursor::new(JSON(json).to_string()));
    response
}
