mod model;

use crate::{CarrierClient, CardStatus, CardInfo};

const API_REST_URL: &str = "https://api.10646.cn/rws/api/v1/";

// 联通帐号密码信息
#[derive(Debug)]
pub struct ChinaUnicomClient<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub soap_license: &'a str,
    pub rest_license: &'a str,
}

impl<'a> ChinaUnicomClient<'a> {
    pub fn new(username: &'a str, password: &'a str, soap_license: &'a str,
        rest_license: &'a str) -> ChinaUnicomClient<'a> {
        ChinaUnicomClient{
            username: username, password: password, soap_license: soap_license,
            rest_license: rest_license,
        }
    }
}

impl<'a> CarrierClient<'a> for ChinaUnicomClient<'a> {
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
