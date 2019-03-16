mod model;

use {std::io::Read, reqwest::Client};
use crate::{Result, CarrierClient, CardStatus, CardInfo};

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
        ChinaTelecomClient{
            username: username.to_owned(),
            password: password.to_owned(),
            license: license.to_owned()
        }
    }
    pub fn get(&self, method: &str, iccid: &str, sign: Vec<&str>, params: Vec<(&str, &str)>) -> String {
        dbg!(self.request(API_GET_URL, method, iccid, sign, params))
    }
    pub fn set(&self, method: &str, iccid: &str, sign: Vec<&str>, params: Vec<(&str, &str)>) -> String {
        dbg!(self.request(API_SET_URL, method, iccid, sign, params))
    }
    fn sign(&self, params: Vec<&str>) -> String {
        let mut data: Vec<&str> = vec![&self.username, &self.password];
        data.extend(params);
        data.sort();
        let data = dbg!(data.join(","));
        let mut iterator = 1;
        let mut remainder = 0;
        let length = data.len();
        if length >= 4 {
            iterator = dbg!(length / 4);
            remainder = dbg!(length % 4);
        };
        data
    }
    fn request(&self, url: &str, method: &str, iccid: &str, sign: Vec<&str>, params: Vec<(&str, &str)>) -> String {
        let mut key = "access_number";
        if iccid.len() == 20 || iccid.len() == 19 {
            key = "iccid";
        };
        let sign_str = self.sign(sign);
        let mut data = vec![("method", method), ("user_id", &self.username), ("passWord", &self.password), ("sign", &sign_str), (key, iccid)];
        data.extend(params);
        let others: Vec<String> = dbg!(data.iter().map(|x| format!("{}={}", x.0, x.1)).collect());
        let url = dbg!(format!("{}?{}",url,others.join("&")));
        let client = Client::new();
        let mut resp = dbg!(client.get(&url).send().unwrap());
        let mut buf = String::new();
        resp.read_to_string(&mut buf).expect("Failed to read response");
        buf
    }
}

impl CarrierClient for ChinaTelecomClient {
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
