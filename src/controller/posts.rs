use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use rocket::request::Form;
use rocket::http::Status;

use ::db;
use ::response::APIResult;
use ::schema::posts::dsl::*;
use ::models::post::{Post, NewPost};

//#[post("/posts", data = "<post>")]
//pub fn write_post(post: Option<Form<NewPost>>, conn: db::Connection) -> APIResult<Post> {
//    post.map_or(APIResult::from_validation_error("Post"), |post| {
//        diesel::insert(post.get()).into(posts)
//            .get_result::<Post>(conn)
//            .map(|new_post| APIResult::new(Status::Created, "Post successfully created", new_post))
//            .unwrap_or_else(|err| APIResult::from_database_error(err, "Post"))
//    })
//}

#[get("/posts")]
pub fn index_post(conn: db::Connection) -> APIResult<Vec<Post>> {
    posts.load::<Post>(&conn as &MysqlConnection)
        .map(|found_posts| APIResult::new(Status::Ok, "Posts successfully indexed", found_posts))
        .unwrap_or_else(|err| APIResult::from_database_error(err, "Post"))
}

#[get("/posts/<post_id>")]
pub fn show_post(post_id: Option<i32>, conn: db::Connection) -> APIResult<Post> {
    post_id.map_or(APIResult::from_validation_error("post_id"), |post_id| {
        posts.find(post_id)
            .first::<Post>(&conn as &MysqlConnection)
            .map(|post| APIResult::new(Status::Ok, "Post successfully shown", post))
            .unwrap_or_else(|err| APIResult::from_database_error(err, "Post"))
    })
}

#[delete("/posts/<post_id>")]
pub fn delete_post(post_id: Option<i32>, conn: db::Connection) -> APIResult<Option<i32>> {
    post_id.map_or(APIResult::from_validation_error("post_id"), |post_id| {
        diesel::delete(posts.find(post_id))
            .execute(&conn as &MysqlConnection)
            .map(|_| APIResult::new(Status::NoContent, "Post successfully deleted", None))
            .unwrap_or_else(|err| APIResult::from_database_error(err, "Post"))
    })
}