mod china_telecom_client;
mod china_unicom_client;
mod china_mobile_client;
mod guangdong_mobile_client;
mod jiangsu_mobile_client;

use crate::client::china_telecom_client::ChinaTelecomClient;
use crate::client::china_unicom_client::ChinaUnicomClient;
use crate::client::china_mobile_client::ChinaMobileClient;

pub trait CarrierClient {
    fn card_status(&self, &'static str) -> String;
    fn card_online_state(&self, &'static str) -> String;
    fn card_info(&self, &'static str) -> String;
    fn card_traffic(&self, &'static str) -> String;
    fn card_plan(&self, &'static str) -> String;
}

pub fn new_carrier(account: &'static str) -> Result<Box<CarrierClient>, &'static str> {
    let v: Vec<&str> = account.split(",").collect();
    match (v[0], v.len()) {
        ("china_telecom", 4) => {
            match v[3].len() {
                9 => Ok(Box::new(ChinaTelecomClient{username: v[1], password: v[2], license: v[3]})),
                _ => Err("不正确的运营商账号"),
            }
        },
        ("china_unicom", 5) => Ok(Box::new(ChinaUnicomClient{username: v[1], password: v[2], soap_license: v[3], rest_license: v[4]})),
        ("china_mobile", 3) => Ok(Box::new(ChinaMobileClient{app_id: v[1], password: v[2]})),
        // ("guangdong_mobile", 4) => Ok(CarrierClient::GuangdongMobile(v[1], v[2], v[3])),
        // ("jiangsu_mobile", 5) => Ok(CarrierClient::JiangsuMobile(v[1], v[2], v[3], v[4])),
        _ => Err("不正确的运营商账号"),
    }
}