use std::fmt;
use std::error;

use diesel::result::Error as DieselError;
use r2d2::{GetTimeout, InitializationError};

use rocket::response;
use rocket::http::Status;
use rocket::response::Responder;

use db::DbError;
use auth::AuthError;
use endpoints::helpers::*;

pub type EndpointResult<T> = Result<T, EndpointError>;

#[derive(Debug)]
pub enum EndpointError {
    Db(DbError),
    Auth(AuthError)
}

impl fmt::Display for EndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EndpointError::Db(ref err) => write!(f, "Db error {}", err),
            EndpointError::Auth(ref err) => write!(f, "Authentication error {}", err),
        }
    }
}

impl error::Error for EndpointError {
    fn description(&self) -> &str {
        match *self {
            EndpointError::Db(ref err) => err.description(),
            EndpointError::Auth(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EndpointError::Db(ref err) => Some(err),
            EndpointError::Auth(ref err) => Some(err),
        }
    }
}

impl From<DbError> for EndpointError {
    fn from(err: DbError) -> EndpointError {
        EndpointError::Db(err)
    }
}

impl From<DieselError> for EndpointError {
    fn from(err: DieselError) -> EndpointError {
        EndpointError::Db(DbError::from(err))
    }
}

impl From<GetTimeout> for EndpointError {
    fn from(err: GetTimeout) -> EndpointError {
        EndpointError::Db(DbError::from(err))
    }
}

impl From<InitializationError> for EndpointError {
    fn from(err: InitializationError) -> EndpointError {
        EndpointError::Db(DbError::from(err))
    }
}

impl From<AuthError> for EndpointError {
    fn from(err: AuthError) -> EndpointError {
        EndpointError::Auth(AuthError::from(err))
    }
}

impl<'r> Responder<'r> for EndpointError {
    fn respond(self) -> response::Result<'r> {
        match self {
            EndpointError::Db(DbError::Db(DieselError::NotFound)) => Ok(not_found_json_response()),
            EndpointError::Auth(_) => Ok(json_response_with_status(Status::Unauthorized, json!({"status": "not_authorized"}))),
            EndpointError::Db(err) => {
                println!("{:?}", err);
                Ok(ise_json_response())
            }
        }
    }
}
