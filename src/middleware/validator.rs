use actix_web::{
    http::{header, HttpTryFrom},
    middleware::{Middleware, Response, Started},
    HttpRequest, HttpResponse, Result,
};
use jwt::{decode, encode, Header, Validation};

use models::Claims;
use error::ERR_TOKEN_IS_NOT_VALID;

pub struct TokenValidator;

impl<S> Middleware<S> for TokenValidator {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        if let Some(h) = req.headers().get("Authorization") {
            if let Ok(t) = h.to_str() {
                if t.len() > 7 {
                    if let Ok(c) =
                        decode::<Claims>(&t[7..], "secret".as_ref(), &Validation::default())
                    {
                        if c.claims.uid == 123456 {
                            return Ok(Started::Done);
                        }
                    }
                }
            }
        }
        return Ok(Started::Response(
            HttpResponse::Ok().json(ERR_TOKEN_IS_NOT_VALID),
        ));
    }
    //
    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        resp.headers_mut().insert(
            header::HeaderName::try_from("X-VERSION").unwrap(),
            header::HeaderValue::from_static("0.2"),
        );
        Ok(Response::Done(resp))
    }
}
