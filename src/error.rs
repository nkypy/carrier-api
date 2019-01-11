use actix_web::{HttpRequest, Json, Result};

use models::{AuthReply, Store};

pub fn http_not_found(_req: &HttpRequest<Store>) -> Result<Json<AuthReply>> {
    Ok(Json(AuthReply {
        error_code: Some(404),
        error_message: Some("NOT FOUND".to_string()),
        token: None,
        data: None,
    }))
}

pub fn http_method_not_allowed(_req: &HttpRequest<Store>) -> Result<Json<AuthReply>> {
    Ok(Json(AuthReply {
        error_code: Some(405),
        error_message: Some("METHOD NOT ALLOWED".to_string()),
        token: None,
        data: None,
    }))
}
