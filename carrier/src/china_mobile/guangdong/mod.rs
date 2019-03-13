mod model;

use crate::{CarrierClient, CardStatus, CardInfo};

const API_URL: &str = "http://120.197.89.173:8081/openapi/router";

// 广东电信帐号信息
#[derive(Debug)]
pub struct GuangdongMobileClient<'a> {
    pub app_id: &'a str,
    pub password: &'a str,
    pub group_code: &'a str,
}

impl<'a> CarrierClient<'a> for GuangdongMobileClient<'a> {
    fn card_status(&self, iccid: &str) -> Result<CardStatus, &'a str> {
        Err("card_status")
    }
    fn card_online(&self, iccid: &str) -> String {
        "card_online".to_string()
    }
    fn card_info(&self, iccid: &str) -> Result<CardInfo, &'a str> {
        Err("card_info")
    }
    fn card_usage(&self, iccid: &str) -> String {
        "card_usage".to_string()
    }
    fn card_plan(&self, iccid: &str) -> String {
        "card_plan".to_string()
    }
}
