use diesel::prelude::*;
use diesel;
use diesel::result::Error as DieselError;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::Post;
use models::NewPost;
use models::UpdatedPost;
use schema::posts::dsl::*;
use schema::posts;
use endpoint_error::{EndpointError, EndpointResult};
use endpoints::helpers::*;

#[get("/posts", format = "application/json")]
fn api_v1_posts_index(db: State<Db>) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    posts.filter(published.eq(false))
        .load::<Post>(conn)
        .map(|results| JSON(json!(results)))
        .map_err(|err| EndpointError::from(err))
}

#[post("/posts", data = "<new_post>", format = "application/json")]
fn api_v1_posts_create(db: State<Db>, new_post: JSON<NewPost>) -> EndpointResult<JSON<Post>> {
    let conn = &*db.pool().get()?;

    diesel::insert(&new_post.0)
        .into(posts::table)
        .get_result::<Post>(conn)
        .map(|post| JSON(post))
        .map_err(|err| EndpointError::from(err))
}

#[get("/posts/<post_id>", format = "application/json")]
fn api_v1_posts_show(post_id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    posts.find(post_id).first::<Post>(conn)
        .and_then(|post| Ok(ok_json_response(json!(post))))
        .or_else(|err| {
            match err {
                DieselError::NotFound => Ok(not_found_json_response()),
                _ => Err(EndpointError::from(err)),
            }
        })
}

#[put("/posts/<post_id>", data = "<updated_post>", format = "application/json")]
fn api_v1_posts_update(db: State<Db>, post_id: i32, updated_post: JSON<UpdatedPost>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::update(posts.find(post_id))
        .set(&updated_post.0)
        .get_result::<Post>(conn)
        .and_then(|post| Ok(ok_json_response(json!(post))))
        .or_else(|err| {
            match err {
                DieselError::NotFound => Ok(not_found_json_response()),
                _ => Err(EndpointError::from(err)),
            }
        })

}

#[delete("/posts/<post_id>", format = "application/json")]
fn api_v1_posts_destroy(post_id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::delete(posts.find(post_id))
        .get_result::<Post>(conn)
        .and_then(|_| Ok(empty_response_with_status(Status::NoContent)))
        .or_else(|err| {
            match err {
                DieselError::NotFound => Ok(not_found_json_response()),
                _ => Err(EndpointError::from(err)),
            }
        })
}

