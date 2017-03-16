use bcrypt::{DEFAULT_COST, hash};
use serde::de::{self, Deserializer, Deserialize};

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[has_many(comments)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="posts"]
pub struct UpdatedPost {
    pub title: Option<String>,
    pub body: Option<String>,
}

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

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="comments"]
pub struct NewComment {
    pub body: String,
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="comments"]
pub struct UpdatedComment {
    pub body: Option<String>,
    pub published: Option<bool>,
    pub user_id: Option<i32>,
    pub post_id: Option<i32>,
}

fn hash_user_password<D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer
{
    let password: String = Deserialize::deserialize(deserializer)?;
    hash(&password, DEFAULT_COST)
        .map_err(de::Error::custom)
        .map(|hashed_password|hashed_password)
}

use super::schema::posts;
use super::schema::users;
use super::schema::comments;
