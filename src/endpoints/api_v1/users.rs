use diesel::prelude::*;
use diesel;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::User;
use models::NewUser;
use models::UpdatedUser;
use schema::posts::dsl::*;
use schema::users::dsl::*;
use schema::users;

use endpoint_error::{EndpointResult};
use endpoints::helpers::*;

#[get("/users", format = "application/json")]
fn api_v1_users_index(db: State<Db>) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    let results = users.load::<User>(conn)?;

    Ok(JSON(json!(results)))
}

#[post("/users", data = "<new_user>", format = "application/json")]
fn api_v1_users_create(db: State<Db>, new_user: JSON<NewUser>) -> EndpointResult<JSON<User>> {
    let conn = &*db.pool().get()?;

    let user = diesel::insert(&new_user.0)
        .into(users::table)
        .get_result::<User>(conn)?;

    Ok(JSON(user))
}

#[get("/users/<id>", format = "application/json")]
fn api_v1_users_show(id: i32, db: State<Db>) -> EndpointResult<JSON<User>> {
    let conn = &*db.pool().get()?;

    let user = users.find(id).first::<User>(conn)?;

    Ok(JSON(user))
}

#[put("/users/<id>", data = "<updated_user>", format = "application/json")]
fn api_v1_users_update(db: State<Db>, id: i32, updated_user: JSON<UpdatedUser>) -> EndpointResult<JSON<User>> {
    let conn = &*db.pool().get()?;

    let user = diesel::update(users.find(id))
        .set(&updated_user.0)
        .get_result::<User>(conn)?;

    Ok(JSON(user))
}

#[delete("/users/<id>", format = "application/json")]
fn api_v1_users_destroy(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::delete(users.find(id)).get_result::<User>(conn)?;

    Ok(empty_response_with_status(Status::NoContent))
}

