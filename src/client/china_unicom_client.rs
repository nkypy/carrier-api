use std::time::Duration;
use actix_web::http::Method;
use actix_web::client::ClientRequest;
use futures::Future;

const CHINA_UNICOM_REST_API_URL: &str = "https://api.10646.cn/rws/api/v1/";

// 联通帐号密码信息
#[derive(Debug)]
pub struct ChinaUnicomClient<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub soap_license: &'a str,
    pub rest_license: &'a str,
}

// 发送短信请求格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChinaUnicomSmsRequest<'a> {
	pub message_text: &'a str,
	pub message_encoding: &'a str,
}

// 返回数据格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChinaUnicomReply<'a> {
    // 错误
    pub error_code: &'a str,
    pub error_message: &'a str,
    // 基本信息
    pub iccid: &'a str,
    pub imsi: &'a str,
    pub msisdn: &'a str,
    pub imei: &'a str,
    pub status: &'a str,
    pub rate_plan: &'a str,
    pub communication_plan: &'a str,
    // info 接口
    pub effective_date: &'a str,
    pub customer: &'a str,
    pub end_cusumer_id: &'a str,
    pub date_activated: &'a str,
    pub date_updated: &'a str,
    pub date_shipped: &'a str,
    pub account_id: &'a str,
    #[serde(rename = "fixedIPAddress")]
    pub fixed_ip_address: &'a str,
    pub sim_notes: &'a str,
    #[serde(rename = "deviceID")]
    pub device_id: &'a str,
    #[serde(rename = "modemID")]
    pub modem_id: &'a str,
    #[serde(rename = "globalSIMType")]
    pub global_sim_type: &'a str,
    // flow 接口
    pub timestamp: &'a str,
    pub cycle_start_date: &'a str,
    pub cycle_end_date: &'a str,
    pub device_cycle_usage_in_zones: &'a str,
    pub data_usage_unit: &'a str,
    pub ctd_data_usage: f64,
    #[serde(rename = "ctdSMSUsage")]
    pub ctd_sms_usage: f64,
    pub ctd_voice_usage: f64,
    pub ctd_session_count: f64,
    pub overage_limit_reached: bool,
    pub overage_limit_override: &'a str,
    // flowUsage 接口
    pub zone: &'a str,
    pub rate_plan_version: &'a str,
    pub data_usage: f64,
    #[serde(rename = "smsmousage")]
    pub sms_mo_usage: f64,
    #[serde(rename = "smsmtusage")]
    pub sms_mt_usage: f64,
    #[serde(rename = "voiceMOUsage")]
    pub voice_mo_usage: f64,
    #[serde(rename = "voiceMOUsageUnit")]
    pub voice_mo_usage_unit: &'a str,
    #[serde(rename = "voiceMTUsage")]
    pub voice_mt_usage: f64,
    #[serde(rename = "voiceMTUsageUnit")]
    pub voice_mt_usage_unit: &'a str,
    // 短信
    pub sms_message_id: i64,
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

impl<'a> ChinaUnicomReply<'a> {
    fn to_card_status(&self) -> String {
        "to_card_status".to_string()
    }
    fn to_card_info(&self) -> String {
        "to_card_info".to_string()
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