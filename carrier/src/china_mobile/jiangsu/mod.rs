mod model;

use {chrono::prelude::Utc, reqwest::Client, serde_xml_rs::to_string};
use crate::{
    CarrierClient, CardStatus, CardInfo,
    china_mobile::jiangsu::model::CardRequest};

const API_URL: &str = "http://221.178.251.182:80/internet_surfing";

// 江苏移动帐号密码信息
#[derive(Debug)]
pub struct JiangsuMobileClient<'a> {
    pub app_id: &'a str,
    pub password: &'a str,
    pub group_code: &'a str,
    pub city_code: &'a str,
}

impl<'a> JiangsuMobileClient<'a> {
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

impl<'a> CarrierClient<'a> for JiangsuMobileClient<'a> {
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