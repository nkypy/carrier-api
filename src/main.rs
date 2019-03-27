#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate hex_literal;

extern crate actix;
extern crate actix_web;
extern crate base64;
extern crate carrier;
extern crate chrono;
extern crate des;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate jsonwebtoken as jwt;
extern crate reqwest;
extern crate serde_xml_rs;
extern crate snowflake;

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
use actix_web::{pred, server, App, HttpRequest, HttpResponse, Json, Path, State};
use chrono::Utc;
use diesel::prelude::*;
use dotenv::dotenv;
use jwt::{decode, encode, Header, Validation};
use std::env;

use crate::api::v1::auth;

use carrier::{
    CarrierClient, ChinaMobileClient, ChinaTelecomClient, ChinaUnicomClient, GuangdongMobileClient,
    JiangsuMobileClient,
};

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
    dotenv().ok();
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "hello-world=debug,carrier=debug,actix_web=info");
    }
    env_logger::init();
    info!("hello world up!");
    // let my_claims = models::Claims {
    //     uid: 123456,
    //     exp: 10000000000,
    // };
    // let key = env::var("JWT_SECRET_KEY").expect("JWT_KEY must be set");
    // let token = match encode(&Header::default(), &my_claims, key.as_ref()) {
    //     Ok(t) => t,
    //     Err(_) => panic!(), // in practice you would return the error
    // };
    // println!("token is {:?}.", token);

    // let carrier = CarrierClient::new("china_telecom,123,456,789789789");
    // match carrier {
    //     Ok(c) => println!("carrier status is {:?}", c.card_status("1234")),
    //     Err(e) => println!("error is {:?}", e)
    // };
    let carrier = ChinaTelecomClient::new(
        &env::var("CHINA_TELECOM_USERNAME").unwrap(),
        &env::var("CHINA_TELECOM_PASSWORD").unwrap(),
        &env::var("CHINA_TELECOM_LICENSE").unwrap(),
    );
    carrier.card_info("8986031630200230821");
    // println!("中国电信 user_id 为 test, password 为 test, key 为 abcdefghi");
    // println!("加密 test 字符串");
    // println!("加密结果 {}", carrier.hash(vec!["test"]));
    // println!("正确结果 {}", "41894168BD86A2CC");
    // println!("加密 [14914000000, test, test, queryPakage] 字符串列表");
    // // 测试循环一万次，是 Go 版本性能的 3 倍左右。
    // let t1 = Utc::now();
    // for i in 0..10000 {
    //     let _x = carrier.hash(vec!["14914000000", "test", "test", "queryPakage"]);
    // }
    // let t2 = Utc::now();
    // println!("时间 {:?}", t2.signed_duration_since(t1));
    // println!(
    //     "加密结果 {}",
    //     carrier.hash(vec!["14914000000", "test", "test", "queryPakage"])
    // );
    // println!("正确结果 {}", "45E8B9924DE397A8F7E5764767810CF774CC7E1685BA702C9C4C367EFDAE5D932B37C0C8F0F8EB0CAD6372289F407CA941894168BD86A2CC32E5804EA05BAA5099649468B9418E52");
    // let carrier = ChinaMobileClient::new(
    //     &env::var("CHINA_MOBILE_APP_ID").unwrap(),
    //     &env::var("CHINA_MOBILE_PASSWORD").unwrap(),
    // );
    // dbg!(carrier.card_status("898602D9981700140197"));
    // let carrier = ChinaUnicomClient::new(
    //     &env::var("CHINA_UNICOM_USERNAME").unwrap(),
    //     &env::var("CHINA_UNICOM_PASSWORD").unwrap(),
    //     "soap_license",
    //     &env::var("CHINA_UNICOM_REST_LICENSE").unwrap(),
    // );
    // dbg!(carrier.card_status("89860117750006390067"));
    // dbg!(ChinaUnicomClient::new_test());
    // let carrier = GuangdongMobileClient::new(
    //     &env::var("GUANGDONG_MOBILE_APP_ID").unwrap(),
    //     &env::var("GUANGDONG_MOBILE_PASSWORD").unwrap(),
    //     &env::var("GUANGDONG_MOBILE_GROUP_ID").unwrap(),
    // );
    // println!("{:?}", carrier.card_status("898602F2191880120110"));
    // let carrier = JiangsuMobileClient::new(
    //     &env::var("JIANGSU_MOBILE_APP_ID").unwrap(),
    //     &env::var("JIANGSU_MOBILE_PASSWORD").unwrap(),
    //     &env::var("JIANGSU_MOBILE_GROUP_ID").unwrap(),
    //     &env::var("JIANGSU_MOBILE_CITY_ID").unwrap(),
    // );
    // carrier.request(
    //     "cc_qryuserinfo",
    //     "",
    //     "",
    //     "01",
    //     "",
    //     "",
    //     "",
    //     "",
    //     "17892100103",
    //     "",
    //     "",
    //     "",
    //     "",
    //     "",
    //     "",
    //     "",
    // );
    // let sys = actix::System::new("hello-world");
    // let bind_address: &str = &env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");
    // info!("api bind address is {}", bind_address);
    // server::new(|| vec![app_state().boxed()])
    //     .bind(bind_address)
    //     .unwrap()
    //     .shutdown_timeout(1)
    //     .start();
    // let _ = sys.run();
}

fn app_state() -> App<models::Store> {
    // let pool = models::establish_connection();
    // App::with_state(models::Store { db: pool.clone() })
    App::with_state(models::Store { db: 1 })
        .middleware(Logger::default())
        // .prefix("/v1/auth") // prefix 后面 default_resource 只对下面的子路由生效，而scope则和group一样
        .scope("/v1", |v1| {
            v1.nested("/auth", |v1auth| {
                // nested 在 scope 下一级
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
                v1card.resource("/status", |r| r.method(Method::GET).f(index))
            })
        })
        .default_resource(|r| {
            // 404
            r.method(Method::GET)
                .f(|_| -> Result<&'static str, Error> { Err(Error::HttpNotFound) });
            // 405
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(|_| -> Result<&'static str, Error> { Err(Error::HttpMethodNotAllowed) });
        })
}
