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
            username: username, password: password, license: license}
    }
    pub fn hash(&self, mut data: Vec<&str>) -> String {
        data.sort();
        data.join(",")
    }
    pub fn get(&self, method: &'a str, iccid: &'a str, sign: &'a str, params: Vec<(&'a str, &'a str)>) -> String {
        "haha".to_string()
    }
    pub fn set(&self, method: &'a str, iccid: &'a str, sign: &'a str, params: Vec<(&'a str, &'a str)>) -> String {
        "setup".to_string()
    }
    pub fn request(&self, url: &'a str, method: &'a str, sign: &'a str, params: Vec<(&'a str, &'a str)>) -> () {
        let url = dbg!(format!("{}?method={}&user_id={}&passWord={}&sign={}",url,method, self.username, self.password,sign));
        let others: Vec<String> = dbg!(params.iter().map(|x| format!("{}={}", x.0, x.1)).collect());
        let url = dbg!(format!("{}&{}",url,others.join("&")));
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
