#[derive(Debug)]
pub struct ChinaTelecomClient<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub license: &'a str,
}

impl<'a> crate::client::CarrierClient for ChinaTelecomClient<'a> {
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