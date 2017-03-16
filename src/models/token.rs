
use models::user::User;
use schema::tokens;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
pub struct Token {
    #[serde(skip_serializing)]
    pub id: i32,
    pub value: String,
    #[serde(skip_serializing)]
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="tokens"]
pub struct NewToken {
    pub value: String,
    pub user_id: i32,
}

