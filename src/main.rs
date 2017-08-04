#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel_demo;
extern crate rocket;
extern crate dotenv;

use std::env;
//use botcket::db;
use dotenv::dotenv;
use botcket::routes::*;

fn main() {
    let _ = dotenv();
    let database_url = env::var("DATABASE_URL").expect("env var DATABASE_URL needed");

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![create_connector, index_connectors, show_connector, delete_connector])
        .manage(db::init_pool(&database_url))
        .launch()
}