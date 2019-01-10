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
mod models;
mod schema;

use actix_web::http::Method;
use actix_web::{server, App, HttpRequest, HttpResponse};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use api::v1::auth;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn index(_req: &HttpRequest) -> HttpResponse {
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
    // let uid = snowflake::ProcessUniqueId::new();
    // let next_uid = snowflake::ProcessUniqueId::new();
    // println!("uid is {:?}, next is {:?}", uid, next_uid);
    let conn = establish_connection();
    let u: Vec<models::User> = schema::users::table
        .load(&conn)
        .expect("Error loading users");
    println!("users are {:?}", u);
    let sys = actix::System::new("hello-world");
    server::new(|| {
        App::new()
            .prefix("/v1/auth")
            .resource("/index", |r| r.method(Method::GET).f(index))
            .resource("/signin", |r| r.method(Method::POST).f(auth::signin))
            .resource("/signup", |r| r.method(Method::POST).with(auth::signup))
            .resource("/signout", |r| r.method(Method::POST).f(auth::signin))
            .resource("/recover", |r| r.method(Method::POST).f(auth::signin))
    }).bind("127.0.0.1:8989")
        .unwrap()
        .shutdown_timeout(1)
        .start();
    let _ = sys.run();
}
