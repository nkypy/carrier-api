static JIANGSU_MOBILE_API_URL: &str = "http://221.178.251.182:80/internet_surfing";

#[derive(Debug)]
pub struct JiangsuMobileClient {
    pub app_id: String,
    pub password: String,
    pub group_code: String,
    pub city_code: String,
}

impl crate::client::CarrierClient for JiangsuMobileClient {
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