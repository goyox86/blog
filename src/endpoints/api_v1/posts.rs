use diesel::prelude::*;
use diesel;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::{Db, DbError};
use models::Post;
use models::NewPost;
use models::UpdatedPost;
use models::User;
use schema::posts::dsl::*;
use schema::posts;
use schema::users::dsl::*;

use endpoint_error::EndpointResult;
use endpoints::helpers::*;
use endpoints::pagination::Pagination;

#[get("/posts", format = "application/json")]
fn index(db: State<Db>) -> EndpointResult<JSON<Value>> {
    let results = published_posts(&*db, None)?;

    Ok(JSON(json!(results)))
}

#[get("/posts?<pagination>", format = "application/json")]
fn index_paginated(db: State<Db>, pagination: Pagination) -> EndpointResult<JSON<Value>> {
    let results = published_posts(&*db, Some(&pagination))?;

    Ok(JSON(json!(results)))
}

#[post("/posts", data = "<new_post>", format = "application/json")]
fn create(db: State<Db>, new_post: JSON<NewPost>) -> EndpointResult<JSON<Post>> {
    let conn = &*db.pool().get()?;

    let post = diesel::insert(&new_post.0).into(posts::table)
        .get_result::<Post>(conn)?;

    Ok(JSON(post))
}

#[get("/posts/<id>", format = "application/json")]
fn show(id: i32, db: State<Db>) -> EndpointResult<JSON<Post>> {
    let conn = &*db.pool().get()?;

    let post = posts.find(id).first::<Post>(conn)?;

    Ok(JSON(post))
}

#[put("/posts/<id>", data = "<updated_post>", format = "application/json")]
fn update(db: State<Db>, id: i32, updated_post: JSON<UpdatedPost>) -> EndpointResult<JSON<Post>> {
    let conn = &*db.pool().get()?;

    let post = diesel::update(posts.find(id)).set(&updated_post.0)
        .get_result::<Post>(conn)?;

    Ok(JSON(post))
}

#[delete("/posts/<id>", format = "application/json")]
fn destroy(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::delete(posts.find(id)).get_result::<Post>(conn)?;

    Response::build().status(Status::NoContent).ok()
}

#[get("/users/<id>/posts", format = "application/json")]
fn user_posts_index(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    let user = users.find(id).first::<User>(conn)?;
    let results = Post::belonging_to(&user).load::<Post>(conn)?;

    Ok(ok_json_response(json!(results)))
}

#[get("/users/<id>/posts/<post_id>", format = "application/json")]
fn user_post_show(id: i32, post_id: i32, db: State<Db>) -> EndpointResult<JSON<Post>> {
    let conn = &*db.pool().get()?;

    let post = posts.filter(user_id.eq(id).and(posts::id.eq(&post_id)))
        .first::<Post>(conn)?;

    Ok(JSON(post))
}

fn published_posts(db: &Db, pagination: Option<&Pagination>) -> Result<Vec<Post>, DbError> {
    let mut query = posts.filter(published.eq(true)).into_boxed();

    if let Some(pagination) = pagination {
        let page = pagination.get_page();
        let per_page = pagination.get_per_page();
        query = query.limit(per_page).offset(per_page * (page - 1));
    }

    let conn = &*db.pool().get()?;

    query.load::<Post>(conn)
        .map_err(DbError::from)
}
