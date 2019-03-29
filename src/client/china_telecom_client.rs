static CHINA_TELECOM_GET_API_URL: &str = "http://api.ct10649.com:9001/m2m_ec/query.do";
static CHINA_TELECOM_SET_API_URL: &str = "http://api.ct10649.com:9001/m2m_ec/app/serviceAccept.do";

#[derive(Debug)]
pub struct ChinaTelecomClient {
    pub username: String,
    pub password: String,
    pub license: String,
}

impl ChinaTelecomClient {
    pub fn hash(&self, mut data: Vec<&str>) -> String {
        data.sort();
        data.join(",")
    }
}

impl crate::client::CarrierClient for ChinaTelecomClient {
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