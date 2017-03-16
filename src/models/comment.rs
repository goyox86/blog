use models::post::Post;
use models::user::User;

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

use schema::comments;