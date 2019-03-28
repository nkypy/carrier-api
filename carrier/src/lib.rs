#![feature(test)]
extern crate test;

mod china_mobile;
mod china_telecom;
mod china_unicom;
mod model;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate iotship_derive;

extern crate base64;
extern crate block_modes;
extern crate chrono;
extern crate des;
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate sha1;
extern crate sha2;

pub use crate::china_mobile::{ChinaMobileClient, GuangdongMobileClient, JiangsuMobileClient};
pub use crate::china_telecom::ChinaTelecomClient;
pub use crate::china_unicom::ChinaUnicomClient;
pub use crate::model::{CardInfo, CardStatus, STATUS_NAME_HASHMAP};

pub type Result<T> = std::result::Result<T, String>;

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
                _ => Err("不正确的运营商账号".to_string()),
            },
            ("china_unicom", 5) => Ok(Box::new(ChinaUnicomClient::new(v[1], v[2], v[3], v[4]))),
            ("china_mobile", 3) => Ok(Box::new(ChinaMobileClient::new(v[1], v[2]))),
            ("guangdong_mobile", 4) => match v[2].len() {
                0..=23 => Err("不正确的运营商账号".to_string()),
                _ => Ok(Box::new(GuangdongMobileClient::new(v[1], v[2], v[3]))),
            },
            ("jiangsu_mobile", 5) => Ok(Box::new(JiangsuMobileClient::new(v[1], v[2], v[3], v[4]))),
            _ => Err("不正确的运营商账号".to_string()),
        }
    }
}
