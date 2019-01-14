use actix_web::{
    http::{header, HttpTryFrom}, middleware::{Middleware, Response, Started}, HttpRequest,
    HttpResponse, Result,
};
use error::AppError;
use jwt::{decode, encode, errors::ErrorKind, Header, Validation};
use models::{Claims, ErrorReply};

pub struct TokenValidator;

impl<S> Middleware<S> for TokenValidator {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let token = match req.headers().get("Authorization") {
            Some(h) => match h.to_str() {
                Ok(t) => match decode::<Claims>(&t[7..], "secret".as_ref(), &Validation::default())
                {
                    Ok(c) => c,
                    Err(_) => {
                        return Ok(Started::Response(HttpResponse::Ok().json(ErrorReply {
                            error_code: 10000404,
                            error_message: "TOKEN NOT VALID".to_string(),
                        })))
                    }
                },
                Err(_) => {
                    return Ok(Started::Response(HttpResponse::Ok().json(ErrorReply {
                        error_code: 10000404,
                        error_message: "TOKEN NOT FOUND".to_string(),
                    })))
                }
            },
            None => {
                return Ok(Started::Response(HttpResponse::Ok().json(ErrorReply {
                    error_code: 10000404,
                    error_message: "TOKEN NOT FOUND".to_string(),
                })))
            }
        };
        if token.claims.uid == 123456 {
            return Ok(Started::Done);
        }
        return Ok(Started::Response(HttpResponse::Ok().json(ErrorReply {
            error_code: 10000404,
            error_message: "TOKEN NOT FOUND".to_string(),
        })));
    }
    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        resp.headers_mut().insert(
            header::HeaderName::try_from("X-VERSION").unwrap(),
            header::HeaderValue::from_static("0.2"),
        );
        Ok(Response::Done(resp))
    }
}
