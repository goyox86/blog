use diesel::prelude::*;
use diesel;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::{Db, DbError};
use models::User;
use models::NewUser;
use models::UpdatedUser;
use schema::users::dsl::*;
use schema::users;

use endpoint_error::EndpointResult;
use endpoints::helpers::*;
use endpoints::pagination::Pagination;

#[get("/users", format = "application/json")]
fn index(db: State<Db>) -> EndpointResult<JSON<Value>> {
    let results = all_users(&db, None)?;

    Ok(JSON(json!(results)))
}

#[get("/users?<pagination>", format = "application/json")]
fn index_paginated(db: State<Db>, pagination: Pagination) -> EndpointResult<JSON<Value>> {
    let results = all_users(&db, Some(pagination))?;

    Ok(JSON(json!(results)))
}

#[post("/users", data = "<new_user>", format = "application/json")]
fn create(db: State<Db>, new_user: JSON<NewUser>) -> EndpointResult<JSON<User>> {
    let conn = &*db.pool().get()?;
  
    let user = diesel::insert(&new_user.0).into(users::table)
        .get_result::<User>(conn)?;

    Ok(JSON(user))
}

#[get("/users/<user_id>", format = "application/json")]
fn show(user_id: i32, db: State<Db>) -> EndpointResult<JSON<User>> {
    let conn = &*db.pool().get()?;

    let user = users.find(user_id).first::<User>(conn)?;

    Ok(JSON(user))
}

#[put("/users/<user_id>", data = "<updated_user>", format = "application/json")]
fn update(db: State<Db>, user_id: i32, updated_user: JSON<UpdatedUser>) -> EndpointResult<JSON<User>> {
    let conn = &*db.pool().get()?;

    let user = diesel::update(users.find(user_id)).set(&updated_user.0)
        .get_result::<User>(conn)?;

    Ok(JSON(user))
}

#[delete("/users/<user_id>", format = "application/json")]
fn destroy(user_id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::delete(users.find(user_id)).get_result::<User>(conn)?;

    Ok(empty_response_with_status(Status::NoContent))
}

fn all_users(db: &Db, pagination: Option<Pagination>) -> Result<Vec<User>, DbError> {
    let mut query = users::table.into_boxed();

    if let Some(pagination) = pagination {
        let page = pagination.get_page();
        let per_page = pagination.get_per_page();
        query = query.limit(per_page).offset(per_page * (page - 1));
    }

    let conn = &*db.pool().get()?;

    query.load::<User>(conn)
        .map_err(|err| DbError::from(err))
}
