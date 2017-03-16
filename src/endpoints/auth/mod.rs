pub mod basic;

use std::error;
use std::fmt;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use bcrypt::{verify, BcryptError};

use db::Db;
use models::User;
use endpoint_error::EndpointResult;
use endpoint_error::EndpointError;

use schema::users::dsl::*;
use schema::users;

#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
    Encryption(BcryptError)
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthError::Unauthorized => write!(f, "could not authenticate the user"),
            AuthError::Encryption(ref err) => write!(f, "there was a problem encrypting/decrypting data {}", err),
        }
    }
}

impl error::Error for AuthError {
    fn description(&self) -> &str {
        match *self {
            AuthError::Unauthorized => "Unauthorized",
            AuthError::Encryption(_) => "Encryption"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AuthError::Unauthorized => Some(self),
            AuthError::Encryption(ref err) => Some(err)
        }
    }
}

pub fn authenticate_user(db: &Db, user_email: &str, password: &str) -> EndpointResult<User> {
    let conn = &*db.pool().get()?;

    let user = match users.filter(users::email.eq(user_email)).first::<User>(conn) {
        Ok(user) => user,
        Err(DieselError::NotFound) => return Err(EndpointError::from(AuthError::Unauthorized)),
        Err(err) => return Err(EndpointError::from(err))
    };

    match verify(password, user.hashed_password.as_ref().expect("hashed password should be there")) {
        Ok(true) => Ok(user),
        Ok(false) => Err(EndpointError::from(AuthError::Unauthorized)),
        Err(err) => Err(EndpointError::from(AuthError::Encryption(err)))
    }
}