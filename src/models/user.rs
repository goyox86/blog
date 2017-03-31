use diesel::prelude::*;
use diesel;
use bcrypt::{DEFAULT_COST, hash};
use serde::de::{self, Deserializer, Deserialize};
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token as JwtToken };

use schema::users;
use schema::comments;
use schema::posts;
use schema::tokens;

use models::token::*;
use db::{Db, DbError};

static AUTH_SECRET: &'static str = "some_secret_key";

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[has_many(posts)]
#[has_many(comments)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub hashed_password: Option<String>
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(rename(deserialize = "password"))]
    #[serde(deserialize_with = "hash_user_password")]
    pub hashed_password: String
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="users"]
pub struct UpdatedUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}

fn hash_user_password<D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer
{
    let password: String = Deserialize::deserialize(deserializer)?;
    hash(&password, DEFAULT_COST)
        .map_err(de::Error::custom)
        .map(|hashed_password|hashed_password)
}

impl User {
    pub fn generate_token(&mut self, db: &Db) -> Result<Token, DbError> {
        let header: Header = Default::default();
        let claims = Registered { sub: Some(self.email.clone()), ..Default::default() };
        let jwt_token = JwtToken::new(header, claims);
        let jwt_token_data = jwt_token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();

        let new_token = NewToken {
            value: jwt_token_data,
            user_id: self.id
        };

        let conn = &*db.pool().get()?;
        diesel::insert(&new_token).into(tokens::table).get_result::<Token>(conn)
            .map_err(DbError::from)
    }

    pub fn find_by_token(db: &Db, token: &str) -> Result<User, DbError> {
        use schema::users::dsl::*;
        use schema::tokens::dsl::*;

        let conn = &*db.pool().get()?;

        let token = tokens.filter(value.eq(token)).first::<Token>(conn)?;

        users.find(token.user_id).first::<User>(conn)
            .map(|user| user)
            .map_err(DbError::from)
    }
}