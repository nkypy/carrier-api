mod model;
mod guangdong;
mod jiangsu;

use {std::io::Read, chrono::Utc, sha2::{Digest, Sha256}, reqwest::Client};
pub use crate::china_mobile::{
    guangdong::GuangdongMobileClient,
    jiangsu::JiangsuMobileClient,
};
use crate::{Result, CarrierClient, CardStatus, CardInfo};

const API_URL: &str = "https://api.iot.10086.cn/v2/";

// 中国移动帐号信息
#[derive(Debug)]
pub struct ChinaMobileClient {
    pub app_id: String,
    pub password: String,
}

impl ChinaMobileClient {
    pub fn new(app_id: &str, password: &str) -> Self {
        ChinaMobileClient{
            app_id: app_id.to_owned(),
            password: password.to_owned()
        }
    }
    pub fn get(&self, method: &str, ebid: &str, params: Vec<(&str, &str)>) -> String {
        let now = Utc::now();
        let trans_id = format!("{}{}{:08}", self.app_id, now.format("%Y%m%d%H%M%S").to_string(), now.timestamp_subsec_nanos());
        let mut hasher = Sha256::new();
        hasher.input(format!("{}{}{}", self.app_id, self.password, trans_id).as_bytes());
        let token = dbg!(format!("{:x}", hasher.result()));
        let mut data: Vec<(&str, &str)> = vec![("appid", &self.app_id), ("ebid", ebid), ("transid", &trans_id), ("token", &token)];
        data.extend(params);
        let others: Vec<String> = dbg!(data.iter().map(|x| format!("{}={}", x.0, x.1)).collect());
        let url = dbg!(format!("{}{}?{}", API_URL, method, others.join("&")));
        let client = Client::new();
        let mut resp = dbg!(client.get(&url).send().unwrap());
        let mut buf = String::new();
        resp.read_to_string(&mut buf).expect("Failed to read response");
        dbg!(buf)
    }
}

impl CarrierClient for ChinaMobileClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus> {
        Err("card_status".to_string())
    }
    fn card_online(&self, iccid: &str) -> String {
        "card_online".to_string()
    }
    fn card_info(&self, iccid: &str) -> Result<CardInfo> {
        Err("card_info".to_string())
    }
    fn card_usage(&self, iccid: &str) -> String {
        "card_usage".to_string()
    }
    fn card_plan(&self, iccid: &str) -> String {
        "card_plan".to_string()
    }
}
