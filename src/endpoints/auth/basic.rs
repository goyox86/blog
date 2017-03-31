use rocket::State;
use rocket_contrib::JSON;
use rocket::request::Form;

use db::Db;
use models::token::Token;
use endpoint_error::EndpointResult;
use auth::authenticate_by_email_password;

#[derive(Deserialize, FromForm)]
pub struct BasicLoginParams {
    pub email: String,
    pub password: String
}

#[post("/basic", data = "<login_params>", format = "application/x-www-form-urlencoded")]
fn login<'r>(db: State<Db>, login_params: Form<BasicLoginParams>) -> EndpointResult<JSON<Token>> {
    let login_params = login_params.into_inner();
    let mut user = authenticate_by_email_password(&db, &login_params.email, &login_params.password)?;

    let token = user.generate_token(&db)?;

    Ok(JSON(token))
}

#[post("/basic", data = "<login_params>", format = "application/json")]
fn login_json(db: State<Db>, login_params: JSON<BasicLoginParams>) -> EndpointResult<JSON<Token>> {
    let mut user = authenticate_by_email_password(&db, &login_params.email, &login_params.password)?;

    let token = user.generate_token(&db)?;

    Ok(JSON(token))
}