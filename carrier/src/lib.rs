mod china_telecom;
mod china_unicom;
mod china_mobile;
mod model;

#[macro_use]
extern crate serde_derive;

use crate::{
    china_telecom::ChinaTelecomClient
};

pub trait CarrierClient<'a> {
    fn card_status(&self, iccid: &'a str) -> String;
    fn card_online(&self, iccid: &'a str) -> String;
    fn card_info(&self, iccid: &'a str) -> String;
    fn card_usage(&self, iccid: &'a str) -> String;
    fn card_plan(&self, iccid: &'a str) -> String;
}

impl<'a> CarrierClient<'a> {
    pub fn new(account: &'static str) -> Result<Box<CarrierClient>, &str> {
        let v: Vec<&str> = account.split(",").collect();
        match (v[0], v.len()) {
            ("china_telecom", 4) => {
                match v[3].len() {
                    9 => Ok(Box::new(ChinaTelecomClient{username: v[1], password: v[2], license: v[3]})),
                    _ => Err("不正确的运营商账号"),
                }
            },
            // ("china_unicom", 5) => Ok(Box::new(ChinaUnicomClient{username: v[1].to_string(), password: v[2].to_string(), soap_license: v[3].to_string(), rest_license: v[4].to_string()})),
            // ("china_mobile", 3) => Ok(Box::new(ChinaMobileClient{app_id: v[1].to_string(), password: v[2].to_string()})),
            // ("guangdong_mobile", 4) => Ok(Box::new(GuangdongMobileClient{app_id: v[1].to_string(), password: v[2].to_string(), group_code: v[3].to_string()})),
            // ("jiangsu_mobile", 5) => Ok(Box::new(JiangsuMobileClient{app_id: v[1].to_string(), password: v[2].to_string(), group_code: v[3].to_string(), city_code: v[4].to_string()})),
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
