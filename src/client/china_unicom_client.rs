#[derive(Debug)]
pub struct ChinaUnicomClient<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub soap_license: &'a str,
    pub rest_license: &'a str,
}

impl<'a> crate::client::CarrierClient for ChinaUnicomClient<'a> {
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