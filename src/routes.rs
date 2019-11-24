use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::{Post, InsertablePost};
use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> String {
    "Hello, world!".to_string()
}

#[get("/<name>")]
pub fn name(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}

#[post("/post", data="<post>")]
pub fn create_post(
    conn: DbConn,
    post: Json<InsertablePost>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::posts::table)
        .values(&post.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/posts")]
pub fn list_posts(conn: DbConn) -> Result<Json<Vec<Post>>, String> {
    use crate::schema::posts::dsl::*;

    posts.load(&conn.0).map_err(|err| -> String {
        println!("Error querying posts: {:?}", err);
        "Error querying posts from the database".into()
    }).map(Json)
}