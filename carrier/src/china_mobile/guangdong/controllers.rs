use block_modes::{block_padding::ZeroPadding, BlockMode, Ecb};
use des::TdesEde3;
use sha1::Sha1;

use super::GuangdongMobileClient;
use crate::Result;

static API_URL: &str = "http://120.197.89.173:8081/openapi/router";

type TdesEcb = Ecb<TdesEde3, ZeroPadding>;

impl GuangdongMobileClient {
    // 解密
    pub fn decrypt(&self, plaintext: String) -> Result<String> {
        Ok(self._tdes_decrypt(self._base64_decode(plaintext)?)?)
    }
    // 3DES 解密, TdesEde3/ECB/ZeroPadding
    fn _tdes_decrypt(&self, mut plaintext: Vec<u8>) -> Result<String> {
        // ECB iv 为默认, CBC 需要提供8位的 iv
        let cipher = TdesEcb::new_var(&self.password.as_bytes()[..24], Default::default())?;
        cipher.decrypt(&mut plaintext)?;
        // 去除无效数据
        plaintext.retain(|&x| x > 20u8);
        Ok(String::from_utf8(plaintext)?)
    }
    // base64 解码
    fn _base64_decode(&self, plaintext: String) -> Result<Vec<u8>> {
        let bytes: Vec<u8> = base64::decode(plaintext.replace('\n', "").as_bytes())?;
        Ok(bytes)
    }
    // 请求 url 生成
    pub fn url(&self, params: Vec<(&str, &str)>) -> String {
        let mut data: Vec<(&str, &str)> =
            vec![("format", "json"), ("v", "3.0"), ("appKey", &self.app_id)];
        data.extend(params);
        data.sort_by(|a, b| a.0.cmp(&b.0));
        let url = data
            .iter()
            .map(|x| format!("{}={}", x.0, x.1))
            .collect::<Vec<String>>()
            .join("&");
        let sign_str = dbg!(format!(
            "{0}{1}{0}",
            self.password,
            data.iter()
                .map(|x| { format!("{}{}", x.0, x.1) })
                .collect::<Vec<String>>()
                .join("")
        ));
        // 签名
        let sign = Sha1::from(&sign_str).digest().to_string().to_uppercase();
        dbg!(format!("{}?sign={}&{}", API_URL, sign, url))
    }
}
