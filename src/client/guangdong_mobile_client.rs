const GUANGDONG_MOBILE_API_URL: &str = "http://120.197.89.173:8081/openapi/router";

#[derive(Debug)]
pub struct GuangdongMobileClient<'a> {
    pub app_id: &'a str,
    pub password: &'a str,
    pub group_code: &'a str,
}

// 返回数据格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuangdongMobileReply<'a> {
    pub code: &'a str,
    pub error: &'a str,
    pub data: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuangdongMobileReplyResult<'a> {
    pub code: &'a str,
    pub desc: &'a str,
    pub order_no: &'a str,
    pub status: &'a str,
    pub status_time: &'a str,
    pub iccid: &'a str,
    pub msisdn: &'a str,
    pub imsi: &'a str,
    pub imei: &'a str,
    pub open_time: &'a str,
    pub low_power_mode: &'a str,
    pub main_prod_code: &'a str,
}

impl<'a> GuangdongMobileReply<'a> {
    fn to_card_status(&self) -> String {
        "to_card_status".to_string()
    }
    fn to_card_info(&self) -> String {
        "to_card_info".to_string()
    }
}

impl<'a> crate::client::CarrierClient for GuangdongMobileClient<'a> {
    fn card_status(&self, iccid: &'static str) -> String {
        "card_status".to_string()
    }
    fn card_online_state(&self, iccid: &'static str) -> String {
        "card_online_state".to_string()
    }
    fn card_info(&self, iccid: &'static str) -> String {
        "card_info".to_string()
    }
    fn card_traffic(&self, iccid: &'static str) -> String {
        "card_traffic".to_string()
    }
    fn card_plan(&self, iccid: &'static str) -> String {
        "card_plan".to_string()
    }
}