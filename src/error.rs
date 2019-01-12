use actix_web::{error::ResponseError, HttpRequest, HttpResponse, Json};

use models::{AuthReply, ErrorReply, Store};

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "http not found")]
    HttpNotFound,
    #[fail(display = "http method not allowed")]
    HttpMethodNotAllowed,
    #[fail(display = "token not found")]
    TokenNotFound,
    #[fail(display = "token validate failed")]
    TokenValidateFailed,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::HttpNotFound => HttpResponse::Ok().json(ErrorReply {
                error_code: 404,
                error_message: "NOT FOUND".to_string(),
            }),
            AppError::HttpMethodNotAllowed => HttpResponse::Ok().json(ErrorReply {
                error_code: 405,
                error_message: "METHOD NOT ALLOWED".to_string(),
            }),
            AppError::TokenNotFound => HttpResponse::Ok().json(ErrorReply {
                error_code: 10000404,
                error_message: "TOKEN NOT FOUND".to_string(),
            }),
            AppError::TokenValidateFailed => HttpResponse::Ok().json(ErrorReply {
                error_code: 10000405,
                error_message: "TOKEN VALIDATE FAILED".to_string(),
            }),
        }
    }
}

pub fn http_not_found(_req: &HttpRequest<Store>) -> Result<Json<AuthReply>, AppError> {
    Err(AppError::HttpNotFound)
}

pub fn http_method_not_allowed(_req: &HttpRequest<Store>) -> Result<Json<AuthReply>, AppError> {
    Err(AppError::HttpMethodNotAllowed)
}
