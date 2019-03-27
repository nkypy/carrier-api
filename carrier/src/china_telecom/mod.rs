mod controller;
mod model;

use reqwest::Client;

use crate::{CardInfo, CardStatus, CarrierClient, Result};

const API_GET_URL: &str = "http://api.ct10649.com:9001/m2m_ec/query.do";
const API_SET_URL: &str = "http://api.ct10649.com:9001/m2m_ec/app/serviceAccept.do";

// 电信帐号信息
#[derive(Debug)]
pub struct ChinaTelecomClient {
    pub username: String,
    pub password: String,
    pub license: String,
}

impl ChinaTelecomClient {
    pub fn new(username: &str, password: &str, license: &str) -> Self {
        ChinaTelecomClient {
            username: username.to_owned(),
            password: password.to_owned(),
            license: license.to_owned(),
        }
    }
    pub fn get(
        &self,
        method: &str,
        iccid: &str,
        signs: Vec<&str>,
        params: Vec<(&str, &str)>,
    ) -> Result<String> {
        dbg!(self.request(API_GET_URL, method, iccid, signs, params))
    }
    pub fn set(
        &self,
        method: &str,
        iccid: &str,
        signs: Vec<&str>,
        params: Vec<(&str, &str)>,
    ) -> Result<String> {
        dbg!(self.request(API_SET_URL, method, iccid, signs, params))
    }
    fn request(
        &self,
        url: &str,
        method: &str,
        iccid: &str,
        signs: Vec<&str>,
        params: Vec<(&str, &str)>,
    ) -> Result<String> {
        let mut key: &str = "access_number";
        if iccid.len() == 20 || iccid.len() == 19 {
            key = "iccid";
        };
        let password_str: String = self.hash(vec![&self.password]);
        // username, password, method 为通用，其他可变
        let mut sign: Vec<&str> = vec![&self.username, &self.password, method];
        sign.extend(signs);
        let sign_str: String = self.hash(sign);
        let mut data: Vec<(&str, &str)> = vec![
            ("method", method),
            ("user_id", &self.username),
            ("passWord", &password_str),
            ("sign", &sign_str),
            (key, iccid),
        ];
        data.extend(params);
        let others: Vec<String> = dbg!(data.iter().map(|x| format!("{}={}", x.0, x.1)).collect());
        let url: String = dbg!(format!("{}?{}", url, others.join("&")));
        Ok(Client::new()
            .get(&url)
            .send()
            .map_err(|_| "超时".to_string())?
            .text()
            .map_err(|_| "读取错误".to_string())?)
    }
}

impl CarrierClient for ChinaTelecomClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus> {
        dbg!(self.get("queryCardMainStatus", iccid, vec![iccid], vec![]));
        Err("card_status".to_string())
    }
    fn card_online(&self, iccid: &str) -> String {
        "card_online".to_string()
    }
    // 接口只能通过 msisdn 查询, TODO
    fn card_info(&self, iccid: &str) -> Result<CardInfo> {
        dbg!(self.get("prodInstQuery", iccid, vec![iccid], vec![]));
        Err("card_info".to_string())
    }
    fn card_usage(&self, iccid: &str) -> String {
        "card_usage".to_string()
    }
    fn card_plan(&self, iccid: &str) -> String {
        "card_plan".to_string()
    }
}
