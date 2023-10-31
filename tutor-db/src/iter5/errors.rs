
use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;


#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}


#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}


impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(_msg) => {
                println!("Database error occurred: {:?}", _msg);
                "Database error".into()
            }
            EzyTutorError::ActixError(_msg) => {
                println!("Server error occurred: {:?}", _msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(_msg) => {
                println!("Not found error occurred: {:?}", _msg);
                _msg.into()
            }
            EzyTutorError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}


impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}


impl From<actix_web::error::Error> for EzyTutorError {
    fn from(err: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(err.to_string())
    }
}


impl From<SQLxError> for EzyTutorError {
    fn from(err: SQLxError) -> Self {
        EzyTutorError::DBError(err.to_string())
    }
}


impl error::ResponseError for EzyTutorError {

    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(_msg) | EzyTutorError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            EzyTutorError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
            EzyTutorError::NotFound(_msg) => StatusCode::NOT_FOUND, }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}