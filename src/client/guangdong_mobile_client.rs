use base64::decode;

static GUANGDONG_MOBILE_API_URL: &str = "http://120.197.89.173:8081/openapi/router";

#[derive(Debug)]
pub struct GuangdongMobileClient {
    pub app_id: String,
    pub password: String,
    pub group_code: String,
}

// 返回数据格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuangdongMobileReply {
    pub code: String,
    pub error: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuangdongMobileReplyResult {
    pub code: String,
    pub desc: String,
    pub order_no: String,
    pub status: String,
    pub status_time: String,
    pub iccid: String,
    pub msisdn: String,
    pub imsi: String,
    pub imei: String,
    pub open_time: String,
    pub low_power_mode: String,
    pub main_prod_code: String,
}

impl GuangdongMobileReply {
    fn to_card_status(&self) -> String {
        "to_card_status".to_string()
    }
    fn to_card_info(&self) -> String {
        "to_card_info".to_string()
    }
}

impl GuangdongMobileClient {
}

impl crate::client::CarrierClient for GuangdongMobileClient {
    fn card_status(&self, iccid: String) -> String {
        "card_status".to_string()
    }
    fn card_online_state(&self, iccid: String) -> String {
        "card_online_state".to_string()
    }
    fn card_info(&self, iccid: String) -> String {
        "card_info".to_string()
    }
    fn card_traffic(&self, iccid: String) -> String {
        "card_traffic".to_string()
    }
    fn card_plan(&self, iccid: String) -> String {
        "card_plan".to_string()
    }
}

pub fn decrypt<'a>(body: &'a str) -> Result<&[u8], &'a str> {
    // let bytes = match decode(body) {
    //     Ok(b) => b,
    //     Err(_) => return Err("解密失败"),
    // };
    let bytes = decode(body).unwrap();
    let c = &bytes[..];
    Ok(c)
}