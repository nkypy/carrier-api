mod model;
mod guangdong;
mod jiangsu;

pub use crate::china_mobile::{
    guangdong::GuangdongMobileClient,
    jiangsu::JiangsuMobileClient,
};
use crate::{CarrierClient, CardStatus, CardInfo};

const API_URL: &str = "https://api.iot.10086.cn/v2/";

// 中国移动帐号信息
#[derive(Debug)]
pub struct ChinaMobileClient<'a> {
    pub app_id: &'a str,
    pub password: &'a str,
}

impl<'a> ChinaMobileClient<'a> {
    pub fn new(app_id: &'a str, password: &'a str) -> ChinaMobileClient<'a> {
        ChinaMobileClient{
            app_id: app_id, password: password,
        }
    }
}

impl<'a> CarrierClient<'a> for ChinaMobileClient<'a> {
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
