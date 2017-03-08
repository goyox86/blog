use diesel::prelude::*;
use diesel;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::Comment;
use models::NewComment;
use models::UpdatedComment;
use models::Post;
use models::User;
use schema::posts::dsl::*;
use schema::comments::dsl::*;
use schema::comments;
use schema::users::dsl::*;

use endpoint_error::EndpointResult;
use endpoints::helpers::*;
use endpoints::pagination::Pagination;


//TODO: Put the common code to get the comments into a reusable function
#[get("/comments", format = "application/json")]
fn index(db: State<Db>) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    let results = comments.filter(comments::published.eq(true))
        .load::<Comment>(conn)?;

    Ok(JSON(json!(results)))
}

#[get("/comments?<pagination>", format = "application/json")]
fn index_paginated(db: State<Db>, pagination: Pagination) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    let page = pagination.get_page();
    let per_page = pagination.get_per_page();

    let results = comments.filter(comments::published.eq(true))
        .limit(per_page)
        .offset(per_page * (page - 1))
        .load::<Comment>(conn)?;

    Ok(JSON(json!(results)))
}

#[post("/comments", data = "<new_comment>", format = "application/json")]
fn create(db: State<Db>, new_comment: JSON<NewComment>) -> EndpointResult<JSON<Comment>> {
    let conn = &*db.pool().get()?;

    let comment = diesel::insert(&new_comment.0).into(comments::table)
        .get_result::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[get("/comments/<id>", format = "application/json")]
fn show(id: i32, db: State<Db>) -> EndpointResult<JSON<Comment>> {
    let conn = &*db.pool().get()?;

    let comment = comments.find(id).first::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[put("/comments/<id>", data = "<updated_comment>", format = "application/json")]
fn update(db: State<Db>,
          id: i32,
          updated_comment: JSON<UpdatedComment>)
          -> EndpointResult<JSON<Comment>> {
    let conn = &*db.pool().get()?;

    let comment = diesel::update(comments.find(id)).set(&updated_comment.0)
        .get_result::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[delete("/comments/<id>", format = "application/json")]
fn destroy(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::delete(comments.find(id)).get_result::<Comment>(conn)?;

    Ok(empty_response_with_status(Status::NoContent))
}

#[get("/posts/<id>/comments", format = "application/json")]
fn post_comments_index(id: i32, db: State<Db>) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    let post = posts.find(id).first::<Post>(conn)?;

    let results = Comment::belonging_to(&post).get_results::<Comment>(conn)?;

    Ok(JSON(json!(results)))
}

#[get("/users/<id>/comments", format = "application/json")]
fn user_comments_index(id: i32, db: State<Db>) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    let user = users.find(id).first::<User>(conn)?;

    let results = Comment::belonging_to(&user).get_results::<Comment>(conn)?;

    Ok(JSON(json!(results)))
}

#[get("/posts/<id>/comments/<comment_id>", format = "application/json")]
fn post_comment_show(id: i32, comment_id: i32, db: State<Db>) -> EndpointResult<JSON<Comment>> {
    let conn = &*db.pool().get()?;

    let comment = comments.filter(post_id.eq(id).and(comments::id.eq(&comment_id)))
        .first::<Comment>(conn)?;

    Ok(JSON(comment))
}
