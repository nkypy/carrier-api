use actix_web::{error::ResponseError, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorReply<'a> {
    pub error_code: usize,
    pub error_message: &'a str,
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

#[derive(Debug, Fail, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Error {
    #[fail(display = "http not found")]
    HttpNotFound,
    #[fail(display = "http method not allowed")]
    HttpMethodNotAllowed,
    #[fail(display = "token is not valid")]
    TokenIsNotValid,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::HttpNotFound => HttpResponse::Ok().json(ERR_HTTP_NOT_FOUND),
            Error::HttpMethodNotAllowed => HttpResponse::Ok().json(ERR_HTTP_METHOD_NOT_ALLOWED),
            Error::TokenIsNotValid => HttpResponse::Ok().json(ERR_TOKEN_IS_NOT_VALID),
            // _ => HttpResponse::Ok().json(ERR_UNKNOWN_ERROR),
        }
    }
}
