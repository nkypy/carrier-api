mod china_telecom;
mod china_unicom;
mod china_mobile;
mod model;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate chrono;
extern crate sha1;
extern crate des;
extern crate reqwest;

pub use crate::model::{CardStatus, CardInfo};
use crate::{
    china_telecom::ChinaTelecomClient, china_unicom::ChinaUnicomClient,
    china_mobile::{
        ChinaMobileClient, GuangdongMobileClient, JiangsuMobileClient},
};

pub trait CarrierClient<'a> {
    fn card_status(&self, iccid: &'a str) -> Result<CardStatus, &'a str>;
    fn card_online(&self, iccid: &'a str) -> String;
    fn card_info(&self, iccid: &'a str) -> Result<CardInfo, &'a str>;
    fn card_usage(&self, iccid: &'a str) -> String;
    fn card_plan(&self, iccid: &'a str) -> String;
}

impl<'a> CarrierClient<'a> {
    pub fn new(account: &'static str) -> Result<Box<CarrierClient>, &str> {
        let v: Vec<&str> = account.split(",").collect();
        match (v[0], v.len()) {
            ("china_telecom", 4) => {
                match v[3].len() {
                    9 => Ok(Box::new(ChinaTelecomClient{
                        username: v[1], password: v[2], license: v[3]})),
                    _ => Err("不正确的运营商账号"),
                }
            },
            ("china_unicom", 5) => Ok(Box::new(ChinaUnicomClient{
                username: v[1], password: v[2], soap_license: v[3], rest_license: v[4]})),
            ("china_mobile", 3) => Ok(Box::new(ChinaMobileClient{
                app_id: v[1], password: v[2]})),
            ("guangdong_mobile", 4) => Ok(Box::new(GuangdongMobileClient{
                app_id: v[1], password: v[2], group_code: v[3]})),
            ("jiangsu_mobile", 5) => Ok(Box::new(JiangsuMobileClient{
                app_id: v[1], password: v[2], group_code: v[3], city_code: v[4]})),
            _ => Err("不正确的运营商账号"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
