pub mod basic;

use std::error;
use std::fmt;
use std::str::FromStr;
use std::str;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use bcrypt::{verify, BcryptError};
use base64::decode as base64_decode;

use db::{Db, DbError};
use models::user::User;
use endpoint_error::EndpointResult;
use endpoint_error::EndpointError;

use schema::users::dsl::*;
use schema::users;

#[derive(Debug)]
pub enum AuthenticationError {
    Unauthorized,
    Encryption(BcryptError)
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthenticationError::Unauthorized => write!(f, "could not authenticate the user"),
            AuthenticationError::Encryption(ref err) => write!(f, "there was a problem encrypting/decrypting data {}", err),
        }
    }
}

impl error::Error for AuthenticationError {
    fn description(&self) -> &str {
        match *self {
            AuthenticationError::Unauthorized => "Unauthorized",
            AuthenticationError::Encryption(_) => "Encryption"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AuthenticationError::Unauthorized => Some(self),
            AuthenticationError::Encryption(ref err) => Some(err)
        }
    }
}

pub struct BasicAuth {
    pub user: String,
    pub password: String
}

pub struct TokenAuth {
    pub token: String
}

pub enum Authentication {
    Basic(BasicAuth),
    Token(TokenAuth)
}

impl Authentication {
    pub fn authenticate(&self, db: &Db) -> EndpointResult<User> {
        match *self {
            Authentication::Basic(ref basic_auth) => authenticate_by_email_password(db, &basic_auth.user, &basic_auth.password),
            Authentication::Token(ref token_auth) => authenticate_by_token(db, &token_auth.token)
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Authentication {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Authentication, ()> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let auth_header_parts: Vec<&str> = auth_header.split_whitespace().collect();
            if auth_header_parts.len() != 2 {
                return Outcome::Failure((Status::BadRequest, ()));
            }

            let authentication = match auth_header_parts[0] {
                "Basic" =>  {
                    let basic_auth = match FromStr::from_str(auth_header_parts[1]) {
                        Ok(basic_auth) => basic_auth,
                        Err(_) => return Outcome::Failure((Status::InternalServerError, ()))
                    };
                    Authentication::Basic(basic_auth)
                },
                "Bearer" => Authentication::Token(TokenAuth { token: auth_header_parts[1].into() }),
                _ =>  return Outcome::Failure((Status::BadRequest, ()))
            };

            Outcome::Success(authentication)
        } else {
            return Outcome::Failure((Status::BadRequest, ()));
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseBasicAuthenticationError(String);

impl FromStr for BasicAuth {
    type Err = ParseBasicAuthenticationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let auth_vec_bytes = match base64_decode(s) {
            Ok(auth_vec_bytes) => auth_vec_bytes,
            Err(_) => return Err(ParseBasicAuthenticationError("error decoding base64".into()))
        };

        let auth_decoded = match String::from_utf8(auth_vec_bytes) {
            Ok(auth_decoded) => auth_decoded,
            Err(_) => return Err(ParseBasicAuthenticationError("utf8 error buing basic auth string".into()))
        };

        let auth_decoded_parts: Vec<&str> = auth_decoded.split(":").collect();

        if auth_decoded_parts.len() != 2 {
            return Err(ParseBasicAuthenticationError("missing user or password".into()));
        }

        let basic_auth = BasicAuth {
            user: auth_decoded_parts[0].into(),
            password: auth_decoded_parts[1].into()
        };

        Ok(basic_auth)
    }
}

pub fn authenticate_by_email_password(db: &Db, user_email: &str, password: &str) -> EndpointResult<User> {
    let conn = &*db.pool().get()?;

    let user = match users.filter(users::email.eq(user_email)).first::<User>(conn) {
        Ok(user) => user,
        Err(DieselError::NotFound) => return Err(EndpointError::from(AuthenticationError::Unauthorized)),
        Err(err) => return Err(EndpointError::from(err))
    };

    match verify(password, user.hashed_password.as_ref().expect("hashed password should be there")) {
        Ok(true) => Ok(user),
        Ok(false) => Err(EndpointError::from(AuthenticationError::Unauthorized)),
        Err(err) => Err(EndpointError::from(AuthenticationError::Encryption(err)))
    }
}

pub fn authenticate_by_token(db: &Db, token: &str) -> EndpointResult<User> {
    User::find_by_token(&db, token)
        .map(|user| user)
        .map_err(|err| match err {
            DbError::Db(DieselError::NotFound) => EndpointError::Auth(AuthenticationError::Unauthorized),
            err => EndpointError::from(err)
        })
}