#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: Option<i32>
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(posts)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String
}

use super::schema::posts;
use super::schema::users;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub user_id: Option<i32>
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub username: &'a str,
}
