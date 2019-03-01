const CHINA_TELECOM_GET_API_URL: &str = "http://api.ct10649.com:9001/m2m_ec/query.do";
const CHINA_TELECOM_SET_API_URL: &str = "http://api.ct10649.com:9001/m2m_ec/app/serviceAccept.do";

#[derive(Debug)]
pub struct ChinaTelecomClient<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub license: &'a str,
}

impl<'a> ChinaTelecomClient<'a> {
    pub fn hash(&self, mut data: Vec<&'a str>) -> String {
        data.sort();
        data.join(",")
    }
}

impl<'a> crate::client::CarrierClient for ChinaTelecomClient<'a> {
    fn card_status(&self, iccid: &'a str) -> String {
        "card_status".to_string()
    }
    fn card_online_state(&self, iccid: &'a str) -> String {
        "card_online_state".to_string()
    }
    fn card_info(&self, iccid: &'a str) -> String {
        "card_info".to_string()
    }
    fn card_traffic(&self, iccid: &'a str) -> String {
        "card_traffic".to_string()
    }
    fn card_plan(&self, iccid: &'a str) -> String {
        "card_plan".to_string()
    }
}