#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

pub mod schema;
pub mod models;
pub mod controller;
pub mod response;
pub mod db;



pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Post {
    use schema::posts::dsl::{posts, id};

    let new_post = models::post::NewPost {
        title: title,
        body: body,
    };

    diesel::insert(&new_post).into(posts)
        .execute(conn)
        .expect("Error saving new post");

    posts.order(id.desc()).first(conn).unwrap()
}