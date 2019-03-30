#![feature(test, async_await, await_macro, futures_api)]
extern crate test;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate futures;

extern crate base64;
extern crate block_modes;
extern crate chrono;
extern crate des;
extern crate hashbrown;
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate sha1;
extern crate sha2;

use std::time::Duration;

mod china_mobile;
mod china_telecom;
mod china_unicom;
mod errors;
mod models;

pub use crate::china_mobile::{ChinaMobileClient, GuangdongMobileClient, JiangsuMobileClient};
pub use crate::china_telecom::ChinaTelecomClient;
pub use crate::china_unicom::ChinaUnicomClient;
pub use crate::errors::Error;
pub use crate::models::{CardInfo, CardStatus, STATUS_NAME_HASHMAP};

pub type Result<T> = std::result::Result<T, Error>;

pub trait CarrierClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus>;
    fn card_online(&self, iccid: &str) -> String;
    fn card_info(&self, iccid: &str) -> Result<CardInfo>;
    fn card_usage(&self, iccid: &str) -> String;
    fn card_plan(&self, iccid: &str) -> String;
}

impl CarrierClient {
    pub fn new(account: &str) -> Result<Box<CarrierClient>> {
        let v: Vec<&str> = account.split(",").collect();
        match (v[0], v.len()) {
            ("china_telecom", 4) => match v[3].len() {
                9 => Ok(Box::new(ChinaTelecomClient::new(v[1], v[2], v[3]))),
                _ => Err("不正确的运营商账号".to_string())?,
            },
            ("china_unicom", 5) => Ok(Box::new(ChinaUnicomClient::new(v[1], v[2], v[3], v[4]))),
            ("china_mobile", 3) => Ok(Box::new(ChinaMobileClient::new(v[1], v[2]))),
            ("guangdong_mobile", 4) => match v[2].len() {
                0..=23 => Err("不正确的运营商账号".to_string())?,
                _ => Ok(Box::new(GuangdongMobileClient::new(v[1], v[2], v[3]))),
            },
            ("jiangsu_mobile", 5) => Ok(Box::new(JiangsuMobileClient::new(v[1], v[2], v[3], v[4]))),
            _ => Err("不正确的运营商账号".to_string())?,
        }
    }
}

// 创建请求运营商的 HTTP 客户端，设置 3 秒超时。
fn http_client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(3))
        .build()?)
}

// 创建异步请求运营商的 HTTP 客户端，设置 3 秒超时。
fn async_http_client() -> Result<reqwest::r#async::Client> {
    Ok(reqwest::r#async::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(3))
        .build()?)
}
