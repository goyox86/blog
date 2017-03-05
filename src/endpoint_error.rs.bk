use std::fmt;
use std::error;

use diesel::result::Error as DieselError;
use r2d2::{GetTimeout, InitializationError};

use rocket::response;
use rocket::response::Responder;

use db::DbError;
use endpoints::helpers::*;

pub type EndpointResult<T> = Result<T, EndpointError>;

#[derive(Debug)]
pub enum EndpointError {
    Db(DbError),
}

impl fmt::Display for EndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EndpointError::Db(ref err) => write!(f, "Db error {}", err),
        }
    }
}

impl error::Error for EndpointError {
    fn description(&self) -> &str {
        match *self {
            EndpointError::Db(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EndpointError::Db(ref err) => Some(err),
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


impl<'r> Responder<'r> for EndpointError {
    fn respond(self) -> response::Result<'r> {
        match self {
            EndpointError::Db(DbError::Db(DieselError::NotFound)) => Ok(not_found_json_response()),
            _ => Ok(ise_json_response())
        }
    }
}
