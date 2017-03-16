use models::user::User;

use schema::posts;
use schema::comments;

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

