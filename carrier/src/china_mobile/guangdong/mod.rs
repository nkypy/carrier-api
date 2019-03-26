mod model;

use base64::decode;
use block_modes::block_padding::ZeroPadding;
use block_modes::{BlockMode, Ecb};
use chrono::Utc;
use des::TdesEde3;
use reqwest::Client;
use sha1::Sha1;

use crate::{CardInfo, CardStatus, CarrierClient, Result};

const API_URL: &str = "http://120.197.89.173:8081/openapi/router";

type TdesEcb = Ecb<TdesEde3, ZeroPadding>;

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
    // 签名, 完成
    pub fn sign(&self, params: Vec<(&str, &str)>) -> (String, String) {
        let mut data: Vec<(&str, &str)> =
            vec![("format", "json"), ("v", "3.0"), ("appKey", &self.app_id)];
        data.extend(params);
        // 排序
        data.sort_by(|a, b| a.0.cmp(&b.0));
        // 拼接 params
        let params_vec: Vec<String> =
            dbg!(data.iter().map(|x| { format!("{}{}", x.0, x.1) }).collect());
        let params_url: Vec<String> = dbg!(data
            .iter()
            .map(|x| { format!("{}={}", x.0, x.1) })
            .collect());
        // 首尾加上 password
        let params_str: String = dbg!(format!("{0}{1}{0}", self.password, params_vec.join("")));
        // Sha1 加密成大写十六进制
        let sign = dbg!(Sha1::from(&params_str).digest().to_string().to_uppercase());
        (params_url.join("&"), sign)
    }
    // 3DES 解密, TdesEde3/ECB/ZeroPadding
    pub fn decrypt(&self, plaintext: &[u8]) -> String {
        let pass = self.password.as_bytes();
        let mut key = [0u8; 24];
        // ECB iv 为默认, CBC 需要提供8位的 iv
        let iv = Default::default();
        key[..24].copy_from_slice(&pass[..24]);
        // iv[..8].copy_from_slice(&pass[..8]);
        let cipher = TdesEcb::new_var(&key, iv).unwrap();
        let pos = plaintext.len();
        let mut buffer = [0u8; 256];
        buffer[..pos].copy_from_slice(plaintext);
        cipher.decrypt(&mut buffer).unwrap();
        let buf = &buffer[..pos];
        String::from_utf8(buf.to_vec())
            .unwrap()
            .replace("\u{6}", "")
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
        let (url, sign) = dbg!(self.sign(v));
        let url = dbg!(format!("{}?sign={}&{}", API_URL, sign, url));
        let rsp = dbg!(Client::new()
            .get(&url)
            .send()
            .map_err(|_| "超时".to_string())?
            .text()
            .map_err(|_| "读取错误".to_string())?);
        Ok(rsp)
    }
}

impl CarrierClient for GuangdongMobileClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus> {
        let data = dbg!(self
            .get("triopi.member.iccid.single.query", iccid)?
            .replace('\n', ""));
        let bytes = decode(&data).unwrap();
        let rsp: &[u8] = &bytes;
        dbg!(self.decrypt(rsp));
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
