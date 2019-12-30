use diesel::{self, prelude::*};
use rocket_contrib::json::Json;
use rand::Rng;

use crate::models::{Quiz, InsertableQuiz};
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

#[post("/post", data="<quiz>")]
pub fn post_new_question(
    conn: DbConn,
    quiz: Json<InsertableQuiz>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::quiz::table)
        .values(&quiz.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/quiz")]
pub fn list_quiz(conn: DbConn) -> Result<Json<Vec<Quiz>>, String> {
    use crate::schema::quiz::dsl::*;

    quiz.load(&conn.0).map_err(|err| -> String {
        println!("Error querying posts: {:?}", err);
        "Error querying posts from the database".into()
    }).map(Json)
}

#[get("/random")]
pub fn random(conn: DbConn) -> Result<Json<Quiz>, String> {
    use crate::schema::quiz::dsl::*;
    let mut rng = rand::thread_rng();

    quiz.load(&conn.0).map_err(|err| -> String {
        println!("Error querying posts: {:?}", err);
        "Error querying posts from the database".into()
    }).map(|mut list| -> Json<Quiz> {
        Json(list.remove(rng.gen_range(0, list.len())))
    })
}