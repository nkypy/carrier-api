#[derive(Debug)]
pub struct JiangsuMobileClient<'a> {
    pub app_id: &'a str,
    pub password: &'a str,
    pub group_code: &'a str,
    pub city_code: &'a str,
}

impl<'a> crate::client::CarrierClient for JiangsuMobileClient<'a> {
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