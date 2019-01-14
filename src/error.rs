use actix_web::{error::ResponseError, HttpRequest, HttpResponse, Json};

use models::{AuthReply, ErrorReply, Store};

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "http not found")]
    HttpNotFound,
    #[fail(display = "http method not allowed")]
    HttpMethodNotAllowed,
    #[fail(display = "token is not valid")]
    TokenIsNotValid,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::HttpNotFound => HttpResponse::Ok().json(ERR_HTTP_NOT_FOUND),
            AppError::HttpMethodNotAllowed => HttpResponse::Ok().json(ERR_HTTP_METHOD_NOT_ALLOWED),
            AppError::TokenIsNotValid => HttpResponse::Ok().json(ERR_TOKEN_IS_NOT_VALID),
            _ => HttpResponse::Ok().json(ERR_UNKNOWN_ERROR),
        }
    }
}

pub fn http_not_found(_req: &HttpRequest<Store>) -> Result<Json<AuthReply>, AppError> {
    Err(AppError::HttpNotFound)
}

pub fn http_method_not_allowed(_req: &HttpRequest<Store>) -> Result<Json<AuthReply>, AppError> {
    Err(AppError::HttpMethodNotAllowed)
}

pub const ERR_HTTP_NOT_FOUND: ErrorReply = ErrorReply {
    error_code: 10000404,
    error_message: "HTTP_NOT_FOUND",
};

pub const ERR_HTTP_METHOD_NOT_ALLOWED: ErrorReply = ErrorReply {
    error_code: 10000405,
    error_message: "HTTP_METHOD_NOT_ALLOWED",
};

pub const ERR_TOKEN_IS_NOT_VALID: ErrorReply = ErrorReply {
    error_code: 10100001,
    error_message: "TOKEN_IS_NOT_VALID",
};

pub const ERR_UNKNOWN_ERROR: ErrorReply = ErrorReply {
    error_code: 10999999,
    error_message: "UNKNOWN_ERROR",
};
