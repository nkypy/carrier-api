const CHINA_MOBILE_API_URL: &str = "https://api.iot.10086.cn/v2/";

#[derive(Debug)]
pub struct ChinaMobileClient {
    pub app_id: String,
    pub password: String,
}

impl crate::client::CarrierClient for ChinaMobileClient {
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