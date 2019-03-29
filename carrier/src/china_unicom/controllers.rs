use reqwest::Client;

use crate::china_unicom::model::RequestEnvelope;
use crate::china_unicom::ChinaUnicomClient;
use crate::Result;

const API_SOAP_URL: &'static str = "https://api.10646.cn/ws/service/terminal";

impl ChinaUnicomClient {
    fn soap_request(&self, method: &str, iccids: Vec<&str>) -> Result<String> {
        let text = dbg!(RequestEnvelope::new(
            &self.username,
            &self.password,
            &self.soap_license,
            method,
            iccids
        ));
        Ok(crate::http_client()?
            .post(API_SOAP_URL)
            .header("Content-Type", "text/xml")
            .header(
                "SOAPAction",
                format!(
                    "http://api.jasperwireless.com/ws/service/terminal/{}",
                    method
                ),
            )
            .body(text)
            .send()
            .map_err(|_| "超时".to_string())?
            .text()
            .map_err(|_| "读取错误".to_string())?)
    }
    pub fn get_terminal_details(&self, iccids: Vec<&str>) -> Result<String> {
        self.soap_request("GetTerminalDetails", iccids)
    }
}
