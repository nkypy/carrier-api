use actix_web::{
    AsyncResponder, FutureResponse, HttpMessage, HttpRequest, HttpResponse, Json, Result,
};
use futures::future::Future;

use models::{AuthReply, AuthRequest};

pub fn signin(req: &HttpRequest) -> FutureResponse<HttpResponse> {
    req.json()
        .from_err()
        .and_then(|auth: AuthRequest| {
            println!("{:?}", auth);
            Ok(HttpResponse::Ok().json(auth))
        })
        .responder()
}

pub fn signup(req: Json<AuthRequest>) -> Result<Json<AuthRequest>> {
    Ok(req)
}
