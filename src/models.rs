#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[has_many(posts)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
}

use super::schema::posts;
use super::schema::users;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub username: String,
}
