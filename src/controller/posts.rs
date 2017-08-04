use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::request::Form;
use rocket::http::Status;

use ::db;
use ::response::APIResult;
use ::schema::connectors::dsl::*;
use ::models::{Connector, NewConnector};

#[post("/connectors", data = "<connector>")]
pub fn create_connector(connector: Option<Form<NewConnector>>, conn: db::Connection) -> APIResult<Connector> {
    connector.map_or(APIResult::from_validation_error("Connector"), |connector| {
        diesel::insert(connector.get()).into(connectors)
            .get_result::<Connector>(&conn as &PgConnection)
            .map(|new_connector| APIResult::new(Status::Created, "Connector successfully created", new_connector))
            .unwrap_or_else(|err| APIResult::from_database_error(err, "Connector"))
    })
}

#[get("/connectors")]
pub fn index_connectors(conn: db::Connection) -> APIResult<Vec<Connector>> {
    connectors.load::<Connector>(&conn as &PgConnection)
        .map(|found_connectors| APIResult::new(Status::Ok, "Connectors successfully indexed", found_connectors))
        .unwrap_or_else(|err| APIResult::from_database_error(err, "Connector"))
}

#[get("/connectors/<connector_id>")]
pub fn show_connector(connector_id: Option<i32>, conn: db::Connection) -> APIResult<Connector> {
    connector_id.map_or(APIResult::from_validation_error("connector_id"), |connector_id| {
        connectors.find(connector_id)
            .first::<Connector>(&conn as &PgConnection)
            .map(|connector| APIResult::new(Status::Ok, "Connector successfully shown", connector))
            .unwrap_or_else(|err| APIResult::from_database_error(err, "Connector"))
    })
}

#[delete("/connectors/<connector_id>")]
pub fn delete_connector(connector_id: Option<i32>, conn: db::Connection) -> APIResult<Option<i32>> {
    connector_id.map_or(APIResult::from_validation_error("connector_id"), |connector_id| {
        diesel::delete(connectors.find(connector_id))
            .execute(&conn as &PgConnection)
            .map(|_| APIResult::new(Status::NoContent, "Connector successfully deleted", None))
            .unwrap_or_else(|err| APIResult::from_database_error(err, "Connector"))
    })
}