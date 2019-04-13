#![feature(test, async_await, await_macro)]
extern crate test;

#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate block_modes;
extern crate chrono;
extern crate des;
extern crate hashbrown;
extern crate lazy_static;
extern crate quick_xml;
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

pub use crate::{
    china_mobile::{ChinaMobileClient, GuangdongMobileClient, JiangsuMobileClient},
    china_telecom::ChinaTelecomClient,
    china_unicom::ChinaUnicomClient,
    errors::Error,
    models::{CardInfo, CardNetStatus, CardRatePlan, CardStatus, CardUsage},
};

pub type Result<T> = std::result::Result<T, Error>;

pub trait CarrierClient {
    // 查询相关
    fn card_status(&self, _: &str) -> Result<CardStatus> {
        Err(("10999901", "暂未支持此接口"))?
    }
    fn card_net_status(&self, _: &str) -> Result<CardNetStatus> {
        Err(("10999901", "暂未支持此接口"))?
    }
    fn card_info(&self, _: &str) -> Result<CardInfo> {
        Err(("10999901", "暂未支持此接口"))?
    }
    fn card_rate_plan(&self, _: &str) -> Result<CardRatePlan> {
        Err(("10999901", "暂未支持此接口"))?
    }
    fn card_usage(&self, _: &str, _: &str) -> Result<CardUsage> {
        Err(("10999901", "暂未支持此接口"))?
    }
    // 设置相关
    fn edit_card_status(&self, _: &str, _: &str) -> Result<String> {
        Err(("10999901", "暂未支持此接口"))?
    }
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
