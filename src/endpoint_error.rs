use std::error::Error;
use std::fmt;

use diesel::result::Error as DieselError;
use r2d2::GetTimeout;

pub type EndpointResult<T> = Result<T, EndpointError>;

#[derive(Debug)]
pub enum EndpointError {
    Db(DieselError),
    DbPool(GetTimeout)
}

impl fmt::Display for EndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EndpointError::Db(ref err) => write!(f, "Db error {}", err),
            EndpointError::DbPool(ref err) => write!(f, "Db pool error {}", err),
        }
    }
}

impl Error for EndpointError {
    fn description(&self) -> &str {
        match *self {
            EndpointError::Db(ref err) => err.description(),
            EndpointError::DbPool(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            EndpointError::Db(ref err) => Some(err),
            EndpointError::DbPool(ref err) => Some(err),
        }
    }
}

impl From<DieselError> for EndpointError {
    fn from(err: DieselError) -> EndpointError {
        EndpointError::Db(err)
    }
}

impl From<GetTimeout> for EndpointError {
    fn from(err: GetTimeout) -> EndpointError {
        EndpointError::DbPool(err)
    }
}
