pub mod basic;


use std::error;
use std::fmt;

use diesel::prelude::*;
use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};

use db::Db;
use models::User;
use endpoint_error::EndpointResult;
use endpoint_error::EndpointError;

#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
    Forbidden,
    Encryption(BcryptError)
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthError::Unauthorized => write!(f, "could not authenticate the user"),
            AuthError::Forbidden => write!(f, "the user does not have access to the resource"),
            AuthError::Encryption(ref err) => write!(f, "there was a problem encryptin/decrypting {}", err),
        }
    }
}

impl error::Error for AuthError {
    fn description(&self) -> &str {
        match *self {
            AuthError::Unauthorized => "Unauthorized",
            AuthError::Forbidden => "Forbidden",
            AuthError::Encryption(_) => "Encryption"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AuthError::Unauthorized => Some(self),
            AuthError::Forbidden => Some(self),
            AuthError::Encryption(ref err) => Some(err)
        }
    }
}

pub fn authenticate_user(db: &Db, email: &str, password: &str) -> EndpointResult<User> {
    use schema::users::dsl::*;
    use schema::users;

    let conn = &*db.pool().get()?;

    let user = users.filter(users::email.eq(email)).first::<User>(conn)?;

    match verify(password, user.hashed_password.as_ref().expect("hashed password should be there")) {
        Ok(true) => Ok(user),
        Ok(false) => Err(EndpointError::from(AuthError::Unauthorized)),
        Err(err) => Err(EndpointError::from(AuthError::Encryption(err)))
    }
}