use std::io::Cursor;
use std::default::Default;

use serde_json;
use serde::Serialize;
use diesel::result::{Error as DieselError, DatabaseErrorKind as DBError};
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::request::Request;

#[derive(Serialize)]
pub struct APIResponse<T: Serialize> {
    pub message: Option<String>,
    pub results: Option<T>,
}

impl<T: Serialize> Default for APIResponse<T> {
    fn default() -> Self {
        APIResponse { message: None, results: None }
    }
}


pub struct APIResult<T: Serialize> {
    status: Status,
    api_result: APIResponse<T>,
}


impl<'r, T: Serialize> Responder<'r> for APIResult<T>{

    fn respond_to(self,req: &Request) -> Result<Response<'r>, Status> {
        let mut response = Response::build();

        serde_json::to_string(&self.api_result)
            .and_then(|serialized| {
                response.status(self.status)
                    .sized_body(Cursor::new(serialized))
                    .header(ContentType::JSON)
                    .ok()
            })
            .map_err(|_| Status::InternalServerError)
    }

}

impl<T: Serialize> APIResult<T> {
    pub fn new(status: Status, message: &str, results: T) -> Self {
        let api_result = APIResponse {
            message: Some(message.to_string()),
            results: Some(results),
        };

        APIResult { status: status, api_result: api_result }
    }

    pub fn from_validation_error(target: &str) -> Self {
        let api_result = APIResponse {
            message: Some(format!("Parameter {} is invalid", target)),
            results: None,
        };

        APIResult {
            status: Status::BadRequest,
            api_result: api_result,
        }
    }

    pub fn from_database_error(err: DieselError, resource: &str) -> Self {
        let mut api_result = APIResponse::default();

        let status = match err {
            DieselError::NotFound => {
                api_result.message = Some(format!("{} not found", resource));
                Status::NotFound
            },
            DieselError::DatabaseError(DBError::UniqueViolation, _) => {
                api_result.message = Some(format!("{} already exists", resource));
                Status::Conflict
            },
            _ => {
                api_result.message = Some(format!("An error occured while handling {}", resource));
                Status::InternalServerError
            },
        };

        APIResult { status: status, api_result: api_result }
    }
}