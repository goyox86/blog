use diesel::prelude::*;
use diesel;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::{Db, DbError};
use models::comment::*;
use models::post::Post;
use models::user::User;
use schema::comments::dsl::*;
use schema::comments;
use schema::users::dsl::*;
use schema::posts::dsl::*;

use endpoint_error::EndpointResult;
use endpoints::helpers::*;
use endpoints::pagination::Pagination;
use auth::Authentication;

#[get("/comments", format = "application/json")]
fn index(auth: Authentication, db: State<Db>) -> EndpointResult<JSON<Value>> {
    auth.authenticate(&db)?;

    let results = all_comments(&db, None)?;

    Ok(JSON(json!(results)))
}

#[get("/comments?<pagination>", format = "application/json")]
fn index_paginated(auth: Authentication, db: State<Db>, pagination: Pagination) -> EndpointResult<JSON<Value>> {
    auth.authenticate(&db)?;

    let results = all_comments(&db, Some(pagination))?;

    Ok(JSON(json!(results)))
}

#[post("/comments", data = "<new_comment>", format = "application/json")]
fn create(auth: Authentication, db: State<Db>, new_comment: JSON<NewComment>) -> EndpointResult<JSON<Comment>> {
    auth.authenticate(&db)?;

    let conn = &*db.pool().get()?;

    let comment = diesel::insert(&new_comment.0).into(comments::table)
        .get_result::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[get("/comments/<id>", format = "application/json")]
fn show(auth: Authentication, id: i32, db: State<Db>) -> EndpointResult<JSON<Comment>> {
    auth.authenticate(&db)?;

    let conn = &*db.pool().get()?;

    let comment = comments.find(id).first::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[put("/comments/<id>", data = "<updated_comment>", format = "application/json")]
fn update(auth: Authentication,
          db: State<Db>,
          id: i32,
          updated_comment: JSON<UpdatedComment>)
          -> EndpointResult<JSON<Comment>> {
    auth.authenticate(&db)?;
    let conn = &*db.pool().get()?;

    let comment = diesel::update(comments.find(id)).set(&updated_comment.0)
        .get_result::<Comment>(conn)?;

    Ok(JSON(comment))
}

#[delete("/comments/<id>", format = "application/json")]
fn destroy(auth: Authentication, id: i32, db: State<Db>) -> EndpointResult<Response> {
    auth.authenticate(&db)?;

    let conn = &*db.pool().get()?;

    diesel::delete(comments.find(id)).get_result::<Comment>(conn)?;

    Ok(empty_response_with_status(Status::NoContent))
}

#[get("/posts/<id>/comments", format = "application/json")]
fn post_comments_index(auth: Authentication, id: i32, db: State<Db>) -> EndpointResult<JSON<Value>> {
    auth.authenticate(&db)?;

    let conn = &*db.pool().get()?;

    let post = posts.find(id).first::<Post>(conn)?;

    let results = Comment::belonging_to(&post).get_results::<Comment>(conn)?;

    Ok(JSON(json!(results)))
}

#[get("/users/<id>/comments", format = "application/json")]
fn user_comments_index(auth: Authentication, id: i32, db: State<Db>) -> EndpointResult<JSON<Value>> {
    auth.authenticate(&db)?;

    let conn = &*db.pool().get()?;

    let user = users.find(id).first::<User>(conn)?;

    let results = Comment::belonging_to(&user).get_results::<Comment>(conn)?;

    Ok(JSON(json!(results)))
}

#[get("/posts/<id>/comments/<comment_id>", format = "application/json")]
fn post_comment_show(auth: Authentication, id: i32, comment_id: i32, db: State<Db>) -> EndpointResult<JSON<Comment>> {
    auth.authenticate(&db)?;

    let conn = &*db.pool().get()?;

    let comment = comments.filter(post_id.eq(id).and(comments::id.eq(&comment_id)))
        .first::<Comment>(conn)?;

    Ok(JSON(comment))
}

fn all_comments(db: &Db, pagination: Option<Pagination>) -> Result<Vec<Comment>, DbError> {
    let mut query = comments.filter(comments::published.eq(true)).into_boxed();

    if let Some(pagination) = pagination {
        let page = pagination.get_page();
        let per_page = pagination.get_per_page();
        query = query.limit(per_page).offset(per_page * (page - 1));
    }

    let conn = &*db.pool().get()?;

    query.load::<Comment>(conn)
        .map_err(|err| DbError::from(err))
}
