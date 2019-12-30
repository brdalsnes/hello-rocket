#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod routes;
pub mod models;
mod schema;
mod cors;

#[database("rocket_app")]
pub struct DbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::index, 
            routes::name,
            routes::post_new_question,
            routes::random])
        .attach(DbConn::fairing())
        .attach(cors::CorsFairing)
        .launch();
}