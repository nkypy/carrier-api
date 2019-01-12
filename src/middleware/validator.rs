use actix_web::{
    http::{header, HttpTryFrom}, middleware::{Middleware, Response, Started}, HttpRequest,
    HttpResponse, Result,
};

pub struct TokenValidator;

impl<S> Middleware<S> for TokenValidator {
    fn start(&self, _req: &HttpRequest<S>) -> Result<Started> {
        Ok(Started::Done)
    }
    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        resp.headers_mut().insert(
            header::HeaderName::try_from("X-VERSION").unwrap(),
            header::HeaderValue::from_static("0.2"),
        );
        Ok(Response::Done(resp))
    }
}
