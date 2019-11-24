
use crate::schema::posts;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Deserialize, Insertable)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub title: String,
    pub body: String,
}