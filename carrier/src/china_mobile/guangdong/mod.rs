mod controllers;
mod models;

use std::str::FromStr;

use chrono::Utc;

use crate::china_mobile::guangdong::models::{CardContent, CardReply};
use crate::{CardInfo, CardStatus, CarrierClient, Result};

// 广东电信帐号信息
#[derive(Debug)]
pub struct GuangdongMobileClient {
    pub app_id: String,
    pub password: String,
    pub group_id: String,
}

impl GuangdongMobileClient {
    pub fn new(app_id: &str, password: &str, group_id: &str) -> Self {
        GuangdongMobileClient {
            app_id: app_id.to_owned(),
            password: password.to_owned(),
            group_id: group_id.to_owned(),
        }
    }
    pub fn get(&self, method: &str, iccid: &str) -> Result<String> {
        let now = Utc::now();
        let nano = format!("{:08}", now.timestamp_subsec_nanos());
        let trans_id = format!(
            "{}{}{}",
            self.group_id,
            now.format("%Y%m%d%H%M%S").to_string(),
            &nano[..7]
        );
        let v = vec![("method", method), ("transID", &trans_id), ("iccid", iccid)];
        let rsp = dbg!(crate::http_client()?.get(&self.url(v)).send()?.text()?);
        dbg!(Ok(self.decrypt(rsp)?))
    }
}

impl CarrierClient for GuangdongMobileClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus> {
        let rsp = self.get("triopi.member.lifecycle.single.query", iccid)?;
        let content = CardContent::from_str(CardReply::from_str(rsp.as_str())?.data.as_str())?;
        Ok(content.into())
    }
}
