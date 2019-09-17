#![allow(dead_code)]

#[macro_use]
extern crate log;

extern crate dotenv;
extern crate carrier;

use dotenv::dotenv;

use std::env;

use carrier::{
    CarrierClient, ChinaMobileClient, ChinaTelecomClient, ChinaUnicomClient, GuangdongMobileClient,
    JiangsuMobileClient,
};

fn main() {
    dotenv().ok();
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "hello-world=debug,carrier=debug");
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
    // let carrier = ChinaTelecomClient::new(
    //     &env::var("CHINA_TELECOM_USERNAME").unwrap(),
    //     &env::var("CHINA_TELECOM_PASSWORD").unwrap(),
    //     &env::var("CHINA_TELECOM_LICENSE").unwrap(),
    // );
    // let rsp = carrier.card_status("8986031630200230821");
    // match rsp {
    //     Ok(t) => {
    //         println!("response is {:#?}", t);
    //         let j = serde_json::to_string(&t).unwrap();
    //         println!("to json {}.", j);
    //     }
    //     Err(e) => println!("error is {}", e),
    // }
    // let carrier = ChinaTelecomClient::new("test", "test", "abcdefghi");
    // // println!("中国电信 user_id 为 test, password 为 test, key 为 abcdefghi");
    // // println!("加密 test 字符串");
    // // println!("加密结果 {}", carrier.hash(vec!["test"]));
    // // println!("正确结果 {}", "41894168BD86A2CC");
    // println!("加密 [14914000000, test, test, queryPakage] 字符串列表");
    // // 测试循环一万次，是 Go 版本性能的 3 倍左右。
    // // 优化代码后，性能是 Go 版本的 20 倍左右。
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
    let carrier = ChinaUnicomClient::new(
        &env::var("CHINA_UNICOM_USERNAME").unwrap(),
        &env::var("CHINA_UNICOM_PASSWORD").unwrap(),
        &env::var("CHINA_UNICOM_SOAP_LICENSE").unwrap(),
        &env::var("CHINA_UNICOM_REST_LICENSE").unwrap(),
    );
    // dbg!(carrier.card_usage("89860918700319648926", "201902"));
    dbg!(carrier.get_terminal_details(vec![
        "89860117750006390067",
        "89860117750006390158",
        "89860117750006390307"
    ]));
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
    // dbg!(carrier.request(
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
    // ));
}
