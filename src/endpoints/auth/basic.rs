use diesel::prelude::*;
use diesel;

use rocket::{State, Response};
use rocket::http::Status;
use rocket_contrib::{JSON, Value};
use rocket::request::Form;
use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};

use db::Db;
use models::User;

use endpoint_error::EndpointResult;
use endpoints::helpers::*;
use endpoints::auth::authenticate_user;

#[derive(Deserialize, FromForm)]
pub struct BasicLoginParams {
    pub email: String,
    pub password: String
}

#[post("/basic", data = "<login_params>", format = "application/x-www-form-urlencoded")]
fn login<'r>(db: State<Db>, login_params: Form<BasicLoginParams>) -> EndpointResult<Response<'r>> {
    use schema::users::dsl::*;
    use schema::users;

    let conn = &*db.pool().get()?;

    let login_params = login_params.into_inner();
    let user = authenticate_user(&db, &login_params.email, &login_params.password)?;

    Ok(ok_json_response(json!({"token": "token"})))
}

#[post("/basic", data = "<login_params>", format = "application/json")]
fn login_json(db: State<Db>, login_params: JSON<BasicLoginParams>) -> EndpointResult<JSON<Value>> {
    Ok(JSON(json!(1)))
}

