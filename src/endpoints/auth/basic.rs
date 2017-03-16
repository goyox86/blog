use std::default::Default;

use rocket::{State, Response};
use rocket_contrib::{JSON, Value};
use rocket::request::Form;

use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};

use db::Db;
use endpoint_error::EndpointResult;
use endpoints::helpers::*;
use auth::authenticate_user;

// TODO Get this from the config
static AUTH_SECRET: &'static str = "some_secret_key";

#[derive(Deserialize, FromForm)]
pub struct BasicLoginParams {
    pub email: String,
    pub password: String
}

#[post("/basic", data = "<login_params>", format = "application/x-www-form-urlencoded")]
fn login<'r>(db: State<Db>, login_params: Form<BasicLoginParams>) -> EndpointResult<Response<'r>> {
    let login_params = login_params.into_inner();
    let user = authenticate_user(&db, &login_params.email, &login_params.password)?;

    let header: Header = Default::default();
    let claims = Registered { sub: Some(user.email), ..Default::default() };
    let token = Token::new(header, claims);
    let jwt = token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();

    Ok(ok_json_response(json!({"token": jwt})))
}

#[post("/basic", data = "<login_params>", format = "application/json")]
fn login_json(db: State<Db>, login_params: JSON<BasicLoginParams>) -> EndpointResult<JSON<Value>> {
    let user = authenticate_user(&db, &login_params.email, &login_params.password)?;

    let header: Header = Default::default();
    let claims = Registered { sub: Some(user.email), ..Default::default() };
    let token = Token::new(header, claims);
    let jwt = token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();

    Ok(JSON(json!({"token": jwt})))
}