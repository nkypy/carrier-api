#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate snowflake;

mod api;
mod error;
mod models;
mod schema;

use actix::prelude::*;
use actix_web::http::Method;
use actix_web::{pred, server, App, HttpRequest, HttpResponse, Path, State};
use diesel::prelude::*;

use api::v1::auth;

fn index(_req: &HttpRequest<models::Store>) -> HttpResponse {
    let reply = models::AuthReply {
        error_code: Some(111),
        error_message: Some("测试错误111".to_string()),
        token: None,
        data: Some(vec![
            models::InfoReply {
                error_code: Some(222),
                error_message: Some("测试错误333".to_string()),
            },
            models::InfoReply {
                error_code: Some(333),
                error_message: Some("测试错误333".to_string()),
            },
        ]),
    };
    HttpResponse::Ok().json(reply)
}

fn main() {
    env_logger::init();
    info!("hello world up!");
    let sys = actix::System::new("hello-world");
    server::new(|| vec![app_state().boxed()])
        .bind("127.0.0.1:8989")
        .unwrap()
        .shutdown_timeout(1)
        .start();
    let _ = sys.run();
}

fn app_state() -> App<models::Store> {
    let pool = models::establish_connection();
    App::with_state(models::Store { db: pool.clone() })
        // .prefix("/v1/auth") // prefix 后面 default_resource 只对下面的子路由生效，而scope则和group一样
        .scope("/v1", |v1| {
            v1
                .nested("/auth", |v1auth| { // nested 在 scope 下一级
                    v1auth
                        .resource("/index", |r| r.method(Method::GET).f(index))
                        .resource("/signin", |r| r.method(Method::POST).with(auth::signup))
                        .resource("/signup", |r| r.method(Method::POST).with(auth::signup))
                        .resource("/signout", |r| r.method(Method::POST).with(auth::signup))
                        .resource("/recover", |r| r.method(Method::POST).with(auth::signup))
                })
                .nested("/user", |v1user| {
                    v1user
                        .resource("/", |r| r.method(Method::GET).f(auth::get_users))
                        .resource("/{username}/{password}", |r| {
                            r.method(Method::GET).with(auth::user_info)
                        })
                })
        })
        .default_resource(|r| {
            // 404
            r.method(Method::GET).f(error::http_not_found);
            // 405
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(error::http_method_not_allowed);
        })
}
