use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use rocket::request::Form;
use rocket::http::Status;

use ::db;
use ::response::APIResult;
use ::schema::posts::dsl::*;
use ::models::post::{Post, NewPost};

#[post("/connectors", data = "<connector>")]
pub fn write_post(connector: Option<Form<NewPost>>, conn: MysqlConnection) -> APIResult<Connector> {
    connector.map_or(APIResult::from_validation_error("Connector"), |connector| {
        diesel::insert(connector.get()).into(connectors)
            .get_result::<Connector>(&conn as &PgConnection)
            .map(|new_connector| APIResult::new(Status::Created, "Connector successfully created", new_connector))
            .unwrap_or_else(|err| APIResult::from_database_error(err, "Connector"))
    })
}

#[get("/connectors")]
pub fn index_post(conn: MysqlConnection) -> APIResult<Vec<Connector>> {
    connectors.load::<Connector>(&conn as &PgConnection)
        .map(|found_connectors| APIResult::new(Status::Ok, "Connectors successfully indexed", found_connectors))
        .unwrap_or_else(|err| APIResult::from_database_error(err, "Connector"))
}

#[get("/connectors/<connector_id>")]
pub fn show_post(connector_id: Option<i32>, conn: MysqlConnection) -> APIResult<Connector> {
    connector_id.map_or(APIResult::from_validation_error("connector_id"), |connector_id| {
        connectors.find(connector_id)
            .first::<Connector>(&conn as &PgConnection)
            .map(|connector| APIResult::new(Status::Ok, "Connector successfully shown", connector))
            .unwrap_or_else(|err| APIResult::from_database_error(err, "Connector"))
    })
}

#[delete("/connectors/<connector_id>")]
pub fn delete_post(connector_id: Option<i32>, conn:MysqlConnection) -> APIResult<Option<i32>> {
    connector_id.map_or(APIResult::from_validation_error("connector_id"), |connector_id| {
        diesel::delete(connectors.find(connector_id))
            .execute(&conn as &PgConnection)
            .map(|_| APIResult::new(Status::NoContent, "Connector successfully deleted", None))
            .unwrap_or_else(|err| APIResult::from_database_error(err, "Connector"))
    })
}