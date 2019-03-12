use crate::client::china_telecom_client::ChinaTelecomClient;
use crate::client::china_unicom_client::ChinaUnicomClient;
use crate::client::china_mobile_client::ChinaMobileClient;
use crate::client::guangdong_mobile_client::GuangdongMobileClient;
use crate::client::jiangsu_mobile_client::JiangsuMobileClient;

pub trait CarrierClient {
    fn card_status(&self, String) -> String;
    fn card_online_state(&self, String) -> String;
    fn card_info(&self, String) -> String;
    fn card_traffic(&self, String) -> String; // 后期改成 card_usage
    fn card_plan(&self, String) -> String;
}

impl CarrierClient {
    pub fn new(account: &str) -> Result<Box<CarrierClient>, &'static str> {
        let v: Vec<&str> = account.split(",").collect();
        match (v[0], v.len()) {
            ("china_telecom", 4) => {
                match v[3].len() {
                    9 => Ok(Box::new(ChinaTelecomClient{username: v[1].to_string(), password: v[2].to_string(), license: v[3].to_string()})),
                    _ => Err("不正确的运营商账号"),
                }
            },
            ("china_unicom", 5) => Ok(Box::new(ChinaUnicomClient{username: v[1].to_string(), password: v[2].to_string(), soap_license: v[3].to_string(), rest_license: v[4].to_string()})),
            ("china_mobile", 3) => Ok(Box::new(ChinaMobileClient{app_id: v[1].to_string(), password: v[2].to_string()})),
            ("guangdong_mobile", 4) => Ok(Box::new(GuangdongMobileClient{app_id: v[1].to_string(), password: v[2].to_string(), group_code: v[3].to_string()})),
            ("jiangsu_mobile", 5) => Ok(Box::new(JiangsuMobileClient{app_id: v[1].to_string(), password: v[2].to_string(), group_code: v[3].to_string(), city_code: v[4].to_string()})),
            _ => Err("不正确的运营商账号"),
        }
    }
}