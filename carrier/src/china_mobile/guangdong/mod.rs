mod model;

use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use des::{block_cipher_trait::BlockCipher, TdesEde3};
use sha1::Sha1;

use crate::{CardInfo, CardStatus, CarrierClient, Result};

const API_URL: &str = "http://120.197.89.173:8081/openapi/router";

type TdesCbc = Cbc<TdesEde3, Pkcs7>;

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
    pub fn sign(&self, params: Vec<(&str, &str)>) -> String {
        let mut data: Vec<(&str, &str)> =
            vec![("format", "json"), ("v", "3.0"), ("appKey", &self.app_id)];
        data.extend(params);
        // 排序
        data.sort_by(|a, b| a.0.cmp(&b.0));
        // 拼接 params
        let params_vec: Vec<String> =
            dbg!(data.iter().map(|x| { format!("{}{}", x.0, x.1) }).collect());
        // 首尾加上 password
        let params_str: String = dbg!(format!("{0}{1}{0}", self.password, params_vec.join("")));
        // Sha1 加密成大写十六进制
        dbg!(Sha1::from(&params_str).digest().to_string().to_uppercase())
    }
    // 3DES 解密, TODO
    pub fn decrypt(&self, plaintext: [u8; 16]) -> () {
        // let key = dbg!(self.password[..24].to_hex());
        let key = dbg!(hex!("000102030405060708090a0b0c0d0e0f1011121314151617"));
        let iv = dbg!(hex!("f0f1f2f3f4f5f6f7"));
        let cipher = TdesCbc::new_var(&key, &iv).unwrap();
        // //
        // let plaintext_in = b"Hello world!";
        // let mut buffer = [0u8; 32];
        // // copy message to the buffer
        // let pos = plaintext_in.len();
        // buffer[..pos].copy_from_slice(plaintext_in);
        // let ciphertext = dbg!(cipher.encrypt(&mut buffer, pos).unwrap());
        // println!("{:x?}", ciphertext);
        //
        let mut buf = dbg!(plaintext.to_vec());
        let decrypted_ciphertext = dbg!(cipher.decrypt(&mut buf).unwrap());
        // dbg!(decrypted_ciphertext);
        dbg!(String::from_utf8(decrypted_ciphertext.to_vec()));
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
