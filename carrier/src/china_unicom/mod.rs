mod controllers;
mod model;

use base64::encode;
use reqwest::Client;

use crate::china_unicom::model::CardReply;
use crate::{CardInfo, CardStatus, CarrierClient, Result};

const API_REST_URL: &str = "https://api.10646.cn/rws/api/v1/";

// 联通帐号密码信息
#[derive(Debug, New)]
pub struct ChinaUnicomClient {
    pub username: String,
    pub password: String,
    pub soap_license: String,
    pub rest_license: String,
    pub rest_auth: String,
}

impl ChinaUnicomClient {
    pub fn new(username: &str, password: &str, soap_license: &str, rest_license: &str) -> Self {
        let rest_auth: String = dbg!(encode(format!("{}:{}", username, rest_license).as_bytes()));
        ChinaUnicomClient {
            username: username.to_owned(),
            password: password.to_owned(),
            soap_license: soap_license.to_owned(),
            rest_license: rest_license.to_owned(),
            rest_auth: rest_auth,
        }
    }
    pub fn get(&self, url: &str) -> Result<String> {
        let url = dbg!(format!("{}{}", API_REST_URL, url));
        dbg!(Ok(Client::new()
            .get(&url)
            .header("Authorization", format!("Basic {}", self.rest_auth))
            .header("Content-Type", "application/json")
            .send()?
            .text()?))
    }
    pub fn put(&self, url: &str, data: &str) -> String {
        "put".to_string()
    }
}

impl CarrierClient for ChinaUnicomClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus> {
        let url = dbg!(format!("devices/{}", iccid));
        let resp = self.get(&url)?;
        let v: CardReply =
            dbg!(serde_json::from_str(&resp).map_err(|_| "解析失败".to_string())?);
        v.to_card_status()
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
