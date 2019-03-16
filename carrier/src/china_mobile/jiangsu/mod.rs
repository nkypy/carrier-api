mod model;

use {chrono::prelude::Utc, reqwest::Client, serde_xml_rs::to_string};
use crate::{
    Result, CarrierClient, CardStatus, CardInfo,
    china_mobile::jiangsu::model::CardRequest};

const API_URL: &str = "http://221.178.251.182:80/internet_surfing";

// 江苏移动帐号密码信息
#[derive(Debug)]
pub struct JiangsuMobileClient {
    pub app_id: String,
    pub password: String,
    pub group_id: String,
    pub city_id: String,
}

impl JiangsuMobileClient {
    pub fn new(app_id: &str, password: &str, group_id: &str, city_id: &str) -> Self {
        JiangsuMobileClient{
            app_id: app_id.to_owned(),
            password: password.to_owned(),
            group_id: group_id.to_owned(),
            city_id: city_id.to_owned()
        }
    }
    fn request(&self) -> () {
        let dt = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let item = CardRequest::new(
            "","","",&dt,"","","","","","","","","","","","","","","","","");
        let data = to_string(&item).unwrap();
        let client = Client::new();
        let resp = client.post(API_URL)
            .body(data)
            .send()
            .unwrap();
        println!("江苏返回 {:?}", resp);
    }
}

impl CarrierClient for JiangsuMobileClient {
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