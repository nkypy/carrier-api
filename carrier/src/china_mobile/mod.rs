mod guangdong;
mod jiangsu;
mod model;

use chrono::Utc;
use sha2::{Digest, Sha256};

pub use crate::china_mobile::guangdong::GuangdongMobileClient;
pub use crate::china_mobile::jiangsu::JiangsuMobileClient;
use crate::china_mobile::model::CardReply;
use crate::{CardInfo, CardStatus, CarrierClient, Result};

static API_URL: &str = "https://api.iot.10086.cn/v2/";

// 中国移动帐号信息
#[derive(Debug)]
pub struct ChinaMobileClient {
    pub app_id: String,
    pub password: String,
}

impl ChinaMobileClient {
    pub fn new(app_id: &str, password: &str) -> Self {
        ChinaMobileClient {
            app_id: app_id.to_owned(),
            password: password.to_owned(),
        }
    }
    pub fn get(&self, method: &str, ebid: &str, params: Vec<(&str, &str)>) -> Result<CardReply> {
        let now = Utc::now();
        let trans_id = format!(
            "{}{}{:08}",
            self.app_id,
            now.format("%Y%m%d%H%M%S").to_string(),
            now.timestamp_subsec_nanos()
        );
        let mut hasher = Sha256::new();
        hasher.input(format!("{}{}{}", self.app_id, self.password, trans_id).as_bytes());
        let token = dbg!(format!("{:x}", hasher.result()));
        let mut data: Vec<(&str, &str)> = vec![
            ("appid", &self.app_id),
            ("ebid", ebid),
            ("transid", &trans_id),
            ("token", &token),
        ];
        data.extend(params);
        let others: Vec<String> = dbg!(data.iter().map(|x| format!("{}={}", x.0, x.1)).collect());
        let url = dbg!(format!("{}{}?{}", API_URL, method, others.join("&")));
        let resp: String = crate::http_client()?.get(&url).send()?.text()?;
        let v: CardReply = dbg!(serde_json::from_str(&resp)?);
        Ok(v)
    }
}

impl CarrierClient for ChinaMobileClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus> {
        let resp = self.get(
            "onandoffrealsingle",
            "0001000000025",
            vec![("iccid", iccid)],
        )?;
        resp.to_card_status()
    }
    fn card_online(&self, iccid: &str) -> String {
        "card_online".to_string()
    }
    fn card_info(&self, iccid: &str) -> Result<CardInfo> {
        Err("card_info".to_string())?
    }
    fn card_usage(&self, iccid: &str) -> String {
        "card_usage".to_string()
    }
    fn card_plan(&self, iccid: &str) -> String {
        "card_plan".to_string()
    }
}
