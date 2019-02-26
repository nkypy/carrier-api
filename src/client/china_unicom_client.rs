use std::time::Duration;
use actix_web::http::Method;
use actix_web::client::ClientRequest;
use futures::Future;

const CHINA_UNICOM_REST_API_URL: &str = "https://api.10646.cn/rws/api/v1/";

#[derive(Debug)]
pub struct ChinaUnicomClient<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub soap_license: &'a str,
    pub rest_license: &'a str,
}

impl<'a> ChinaUnicomClient<'a> {
    pub fn rest_request(&self, url: &'a str, method: Method) -> String {
        let mut builder = ClientRequest::build();
        builder
            .uri(format!("{}{}", CHINA_UNICOM_REST_API_URL, url))
            .method(method)
            .header("Authorization", format!("Basic {}", base64::encode(format!("{}:{}", self.username, self.rest_license).as_bytes())))
            .header("Content-Type", "application/json")
            .header("User-Agent", "actix_web client")
            .finish()
            .unwrap()
            .send()
            .timeout(Duration::from_secs(10))                               // <- Send http request
            .map_err(|_| ())
            .and_then(|response| {                // <- server http response
                println!("Response: {:?}", response);
                Ok(())
            });
        "结束".to_string()
    }
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