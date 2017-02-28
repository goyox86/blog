use diesel::prelude::*;
use diesel;
use diesel::result::Error as DieselError;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};

use db::Db;
use models::User;
use models::NewUser;
use models::UpdatedUser;
use schema::posts::dsl::*;
use schema::posts;
use schema::users::dsl::*;
use schema::users;

use endpoint_error::{EndpointError, EndpointResult};
use endpoints::helpers::*;

#[get("/users", format = "application/json")]
fn api_v1_users_index(db: State<Db>) -> EndpointResult<JSON<Value>> {
    let conn = &*db.pool().get()?;

    users.load::<User>(conn)
        .map(|results| JSON(json!(results)))
        .map_err(|err| EndpointError::from(err))
}

#[post("/users", data = "<new_user>", format = "application/json")]
fn api_v1_users_create(db: State<Db>, new_user: JSON<NewUser>) -> EndpointResult<JSON<User>> {
    let conn = &*db.pool().get()?;

    diesel::insert(&new_user.0)
        .into(users::table)
        .get_result::<User>(conn)
        .map(|user| JSON(user))
        .map_err(|err| EndpointError::from(err))
}

#[get("/users/<id>", format = "application/json")]
fn api_v1_users_show(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    users.find(id).first::<User>(conn)
        .and_then(|user| Ok(ok_json_response(json!(user))))
        .or_else(|err| {
            match err {
                DieselError::NotFound => Ok(not_found_json_response()),
                _ => Err(EndpointError::from(err)),
            }
        })
}

// FIXME: This should behave like a Rails update only update the fields passed in the payload.
#[put("/users/<id>", data = "<updated_user>", format = "application/json")]
fn api_v1_users_update(db: State<Db>, id: i32, updated_user: JSON<UpdatedUser>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::update(users.find(id))
        .set(&updated_user.0)
        .get_result::<User>(conn)
        .and_then(|user| Ok(ok_json_response(json!(user))))
        .or_else(|err| {
            match err {
                DieselError::NotFound => Ok(not_found_json_response()),
                _ => Err(EndpointError::from(err)),
            }
        })

}

#[delete("/users/<id>", format = "application/json")]
fn api_v1_users_destroy(id: i32, db: State<Db>) -> EndpointResult<Response> {
    let conn = &*db.pool().get()?;

    diesel::delete(users.find(id))
        .get_result::<User>(conn)
        .and_then(|_| Ok(empty_response_with_status(Status::NoContent)))
        .or_else(|err| {
            match err {
                DieselError::NotFound => Ok(not_found_json_response()),
                _ => Err(EndpointError::from(err)),
            }
        })
}

