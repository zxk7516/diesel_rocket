#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel_demo;
extern crate rocket;
extern crate dotenv;

use std::env;
use dotenv::dotenv;
use diesel_demo::*;

fn main() {
    let _ = dotenv();
    let database_url = env::var("DATABASE_URL").expect("env var DATABASE_URL needed");

    let db_conn = db::establish_connection();

    rocket::ignite()
        .mount("/", routes![index])
        .manage(db_conn)
        .launch();
}