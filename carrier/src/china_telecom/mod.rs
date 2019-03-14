mod model;

use crate::{CarrierClient, CardStatus, CardInfo};

const API_GET_URL: &str = "http://api.ct10649.com:9001/m2m_ec/query.do";
const API_SET_URL: &str = "http://api.ct10649.com:9001/m2m_ec/app/serviceAccept.do";

// 电信帐号信息
#[derive(Debug)]
pub struct ChinaTelecomClient<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub license: &'a str,
}

impl<'a> ChinaTelecomClient<'a> {
    pub fn new(username: &'a str, password: &'a str, license: &'a str) -> ChinaTelecomClient<'a> {
        ChinaTelecomClient{
            username: username, password: password, license: license,
        }
    }
    pub fn get(&self, method: &'a str, iccid: &'a str, sign: Vec<&'a str>, params: Vec<(&'a str, &'a str)>) -> String {
        dbg!(self.request(API_GET_URL, method, iccid, sign, params))
    }
    pub fn set(&self, method: &'a str, iccid: &'a str, sign: Vec<&'a str>, params: Vec<(&'a str, &'a str)>) -> String {
        dbg!(self.request(API_SET_URL, method, iccid, sign, params))
    }
    fn sign(&self, params: Vec<&'a str>) -> String {
        let mut data = vec![self.username, self.password];
        data.extend(params);
        data.sort();
        dbg!(data.join(","))
    }
    fn request(&self, url: &'a str, method: &'a str, iccid: &'a str, sign: Vec<&'a str>, params: Vec<(&'a str, &'a str)>) -> String {
        let mut key = "access_number";
        if iccid.len() == 20 || iccid.len() == 19 {
            key = "iccid";
        };
        let sign_str = self.sign(sign);
        let mut data = vec![("method", method), ("user_id", self.username), ("passWord", self.password), ("sign", &sign_str), (key, iccid)];
        data.extend(params);
        let others: Vec<String> = dbg!(data.iter().map(|x| format!("{}={}", x.0, x.1)).collect());
        format!("{}?{}",url,others.join("&"))
    }
}

impl<'a> CarrierClient<'a> for ChinaTelecomClient<'a> {
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
