mod controllers;
mod models;

use std::str::FromStr;

use crate::china_telecom::models::{CardMsisdnReply, CardStatusReply};
use crate::{CardInfo, CardStatus, CarrierClient, Result};

static API_GET_URL: &str = "http://api.ct10649.com:9001/m2m_ec/query.do";
static API_SET_URL: &str = "http://api.ct10649.com:9001/m2m_ec/app/serviceAccept.do";

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
        self.request(API_GET_URL, method, iccid, signs, params)
    }
    pub fn set(
        &self,
        method: &str,
        iccid: &str,
        signs: Vec<&str>,
        params: Vec<(&str, &str)>,
    ) -> Result<String> {
        self.request(API_SET_URL, method, iccid, signs, params)
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
        Ok(crate::http_client()?.get(&url).send()?.text()?)
    }
    fn iccid_to_msisdn(&self, iccid: &str) -> Result<String> {
        let resp = dbg!(self.get("getTelephone", iccid, vec![iccid], vec![])?);
        Ok(CardMsisdnReply::from_str(&resp)?.msisdn)
    }
}

impl CarrierClient for ChinaTelecomClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus> {
        let resp = dbg!(self.get("queryCardMainStatus", iccid, vec![iccid], vec![]))?;
        // let v: CardStatusReply = serde_json::from_str(&resp)?;
        // dbg!(v.to_card_status())
        Ok(CardStatusReply::from_str(&resp)?.into())
    }
    // 接口只能通过 msisdn 查询
    fn card_info(&self, iccid: &str) -> Result<CardInfo> {
        let msisdn = self.iccid_to_msisdn(iccid)?;
        dbg!(self.get("prodInstQuery", &msisdn, vec![&msisdn], vec![]));
        Err("card_info".to_string())?
    }
}
