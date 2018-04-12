use actix_web::HttpRequest;

pub fn index(req: HttpRequest) -> &'static str {
    "Hello, 世界！"
}
