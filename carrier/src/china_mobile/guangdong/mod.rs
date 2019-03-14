mod model;

use {sha1::Sha1, des::TdesEde3};
use crate::{CarrierClient, CardStatus, CardInfo};

const API_URL: &str = "http://120.197.89.173:8081/openapi/router";

// 广东电信帐号信息
#[derive(Debug)]
pub struct GuangdongMobileClient<'a> {
    pub app_id: &'a str,
    pub password: &'a str,
    pub group_id: &'a str,
}

impl<'a> GuangdongMobileClient<'a> {
    pub fn new(app_id: &'a str, password: &'a str, group_id: &'a str) -> GuangdongMobileClient<'a> {
        GuangdongMobileClient{
            app_id: app_id, password: password, group_id: group_id,
        }
    }
    // 签名, 完成
    pub fn sign(&self, mut params: Vec<(&'a str, &'a str)>) -> String {
        let params_init: Vec<(&'a str, &'a str)> = vec![("format", "json"), ("v", "3.0"), ("appKey", self.app_id)];
        params.extend(params_init);
        // 排序
        params.sort_by(|a, b| a.0.cmp(&b.0));
        // 拼接 params
        let params_vec: Vec<String> = dbg!(params.iter().map(|x| { format!("{}{}", x.0, x.1) }).collect());
        // 首尾加上 password
        let params_str: String = dbg!(format!("{0}{1}{0}", self.password, params_vec.join("")));
        // Sha1 加密成大写十六进制
        dbg!(Sha1::from(&params_str).digest().to_string().to_uppercase())
    }
    // 3DES 解密, TODO
    fn decrypt(&self) -> () {
        let key = &self.password[..24];
    }
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
