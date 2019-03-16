#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

extern crate base64;
extern crate des;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate jsonwebtoken as jwt;
extern crate snowflake;
extern crate serde_xml_rs;
extern crate reqwest;
extern crate carrier;
extern crate chrono;

mod api;
mod error;
mod middleware;
mod models;
mod schema;
// mod client;

use error::Error;

use actix::prelude::*;
use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::{pred, server, App, HttpRequest, HttpResponse, Path, Json, State};
use diesel::prelude::*;
use jwt::{decode, encode, Header, Validation};
use std::env;
use dotenv::dotenv;

use crate::api::v1::auth;

use carrier::{CarrierClient, ChinaTelecomClient, ChinaUnicomClient, ChinaMobileClient, GuangdongMobileClient};

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
    dotenv().ok();
    let my_claims = models::Claims {
        uid: 123456,
        exp: 10000000000,
    };
    let key = env::var("JWT_KEY").expect("JWT_KEY must be set");
    let token = match encode(&Header::default(), &my_claims, key.as_ref()) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    println!("token is {:?}.", token);

    // let carrier = CarrierClient::new("china_telecom,123,456,789789789");
    let carrier = CarrierClient::new("guangdong_mobile,123,456123123123123123123123,789");
    // if let Ok(c) = carrier {
    //     println!("carrier status is {:?}", c.card_status("1234"));
    // }
    match carrier {
        Ok(c) => println!("carrier status is {:?}", c.card_status("1234")),
        Err(e) => println!("error is {:?}", e)
    };
    // let carrier = ChinaTelecomClient::new("123", "456", "789");
    // carrier.get("test", "12345678901234567890", vec!["signValue"], vec![("test_name", "test_value"), ("test2.1", "test2.2")]);
    // let carrier = ChinaMobileClient::new("123", "456");
    // carrier.get("gprsrealsingle", "0001000000000", vec![("iccid", "898602D9981700140197")]);
    let carrier = ChinaUnicomClient::new(
        &env::var("CHINA_UNICOM_USERNAME").unwrap(),
        &env::var("CHINA_UNICOM_PASSWORD").unwrap(),
        "soap_license",
        &env::var("CHINA_UNICOM_REST_LICENSE").unwrap()
    );
    dbg!(carrier.card_status("89860117750006390067"));
    // let carrier = GuangdongMobileClient::new("123","8493fed21155dddd67c2aaa95aaebd11","789");
    // carrier.sign(vec![("haha", "hoho")]);
    // carrier.decrypt();
    // let s = crate::client::decrypt("MTIzCg==");
    // println!("base64 is {:?}", s);
    let sys = actix::System::new("hello-world");
    let server_name: &str = "0.0.0.0:8989";
    info!("server is running at {}", server_name);
    server::new(|| vec![app_state().boxed()])
        .bind(server_name)
        .unwrap()
        .shutdown_timeout(1)
        .start();
    let _ = sys.run();
}

fn app_state() -> App<models::Store> {
    // let pool = models::establish_connection();
    // App::with_state(models::Store { db: pool.clone() })
    App::with_state(models::Store{db: 1})
        .middleware(Logger::default())
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
                        .middleware(middleware::validator::TokenValidator)
                        .resource("/", |r| r.method(Method::GET).f(auth::get_users))
                        .resource("/{username}/{password}", |r| {
                            r.method(Method::GET).with(auth::user_info)
                        })
                })
                .nested("/card/{iccid}", |v1card| {
                    v1card
                        .resource("/status", |r| r.method(Method::GET).f(index))
                })
        })
        .default_resource(|r| {
            // 404
            r.method(Method::GET).f(|_| -> Result<&'static str, Error> {Err(Error::HttpNotFound)});
            // 405
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(|_| -> Result<&'static str, Error> {Err(Error::HttpMethodNotAllowed)});
        })
}
