use diesel::prelude::*;
use diesel;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::Post;
use models::NewPost;
use models::UpdatedPost;
use models::User;
use schema::posts::dsl::*;
use schema::posts;
use schema::users::dsl::*;

use endpoint_error::EndpointResult;
use endpoints::helpers::*;

#[get("/posts", format = "application/json")]
fn api_v1_posts_index(db: State<Db>) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    let results = posts.filter(published.eq(false))
        .load::<Post>(conn)?;

    Ok(JSON(json!(results)))
}

#[post("/posts", data = "<new_post>", format = "application/json")]
fn api_v1_posts_create(db: State<Db>, new_post: JSON<NewPost>) -> EndpointResult<JSON<Post>> {
    let conn = &*db.pool().get()?;

    let post = diesel::insert(&new_post.0)
        .into(posts::table)
        .get_result::<Post>(conn)?;

    Ok(JSON(post))
}

#[get("/posts/<id>", format = "application/json")]
fn api_v1_posts_show(id: i32, db: State<Db>) -> EndpointResult<JSON<Post>> {
    let conn = &*db.pool().get()?;

    let post = posts.find(id).first::<Post>(conn)?;

    Ok(JSON(post))
}

#[put("/posts/<id>", data = "<updated_post>", format = "application/json")]
fn api_v1_posts_update(db: State<Db>, id: i32, updated_post: JSON<UpdatedPost>) -> EndpointResult<JSON<Post>> {
    let conn = &*db.pool().get()?;

    let post = diesel::update(posts.find(id))
        .set(&updated_post.0)
        .get_result::<Post>(conn)?;

    Ok(JSON(post))
}

#[delete("/posts/<id>", format = "application/json")]
fn api_v1_posts_destroy(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::delete(posts.find(id)).execute(conn)?;

    Ok(empty_response_with_status(Status::NoContent))
}

#[get("/users/<id>/posts", format = "application/json")]
fn api_v1_users_posts_index(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    let user = users.find(id).first::<User>(conn)?;
    let results = Post::belonging_to(&user).load::<Post>(conn)?;

    Ok(ok_json_response(json!(results)))
}

