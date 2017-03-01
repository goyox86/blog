use diesel::prelude::*;
use diesel;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::Comment;
use models::NewComment;
use models::UpdatedComment;
use schema::comments::dsl::*;
use schema::comments;
use schema::users::dsl::*;
use schema::users;

use endpoint_error::EndpointResult;
use endpoints::helpers::*;

#[get("/comments", format = "application/json")]
fn api_v1_comments_index(db: State<Db>) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    let results = comments.filter(published.eq(false))
        .load::<Comment>(conn)?;

    Ok(JSON(json!(results)))
}

#[post("/comments", data = "<new_comment>", format = "application/json")]
fn api_v1_comments_create(db: State<Db>, new_comment: JSON<NewComment>) -> EndpointResult<JSON<Comment>> {
    let conn = &*db.pool().get()?;

    let comment = diesel::insert(&new_comment.0)
        .into(comments::table)
        .get_result::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[get("/comments/<id>", format = "application/json")]
fn api_v1_comments_show(id: i32, db: State<Db>) -> EndpointResult<JSON<Comment>> {
    let conn = &*db.pool().get()?;

    let comment = comments.find(id).first::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[put("/comments/<id>", data = "<updated_comment>", format = "application/json")]
fn api_v1_comments_update(db: State<Db>, id: i32, updated_comment: JSON<UpdatedComment>) -> EndpointResult<JSON<Comment>> {
    let conn = &*db.pool().get()?;

    let comment = diesel::update(comments.find(id))
        .set(&updated_comment.0)
        .get_result::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[delete("/comments/<id>", format = "application/json")]
fn api_v1_comments_destroy(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::delete(comments.find(id)).get_result::<Comment>(conn)?;

    Ok(empty_response_with_status(Status::NoContent))
}

