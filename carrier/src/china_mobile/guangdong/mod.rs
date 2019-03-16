mod model;

use {
    sha1::Sha1,
    generic_array::{GenericArray, typenum::U8},
    des::{
        block_cipher_trait::BlockCipher,
        TdesEde3,
    },
};
use crate::{Result, CarrierClient, CardStatus, CardInfo};

const API_URL: &str = "http://120.197.89.173:8081/openapi/router";

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
            group_id: group_id.to_owned()
        }
    }
    // 签名, 完成
    pub fn sign(&self, params: Vec<(&str, &str)>) -> String {
        let mut data: Vec<(&str, &str)> = vec![("format", "json"), ("v", "3.0"), ("appKey", &self.app_id)];
        data.extend(params);
        // 排序
        data.sort_by(|a, b| a.0.cmp(&b.0));
        // 拼接 params
        let params_vec: Vec<String> = dbg!(data.iter().map(|x| { format!("{}{}", x.0, x.1) }).collect());
        // 首尾加上 password
        let params_str: String = dbg!(format!("{0}{1}{0}", self.password, params_vec.join("")));
        // Sha1 加密成大写十六进制
        dbg!(Sha1::from(&params_str).digest().to_string().to_uppercase())
    }
    // 3DES 解密, TODO
    pub fn decrypt(&self) -> () {
        let a = dbg!(self.password[..24].as_bytes());
        let x = GenericArray::from_slice(&a);
        let t = dbg!(TdesEde3::new(&x));
        let mut c = GenericArray::<u8, U8>::default();
        let mut data = GenericArray::from_mut_slice(&mut c);
        t.encrypt_block(data);
        dbg!(data.as_slice());
    }
    pub fn get(&self) -> String {
        "get".to_string()
    }
}

impl CarrierClient for GuangdongMobileClient {
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
