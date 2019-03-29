use std::time::Duration;
use actix_web::http::Method;
use futures::Future;
use reqwest::{ClientBuilder, Request, Url};

static CHINA_UNICOM_REST_API_URL: &str = "https://api.10646.cn/rws/api/v1/";

// 联通帐号密码信息
#[derive(Debug)]
pub struct ChinaUnicomClient {
    pub username: String,
    pub password: String,
    pub soap_license: String,
    pub rest_license: String,
}

// 发送短信请求格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChinaUnicomSmsRequest {
	pub message_text: String,
	pub message_encoding: String,
}

// 返回数据格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChinaUnicomReply {
    // 错误
    pub error_code: String,
    pub error_message: String,
    // 基本信息
    pub iccid: String,
    pub imsi: String,
    pub msisdn: String,
    pub imei: String,
    pub status: String,
    pub rate_plan: String,
    pub communication_plan: String,
    // info 接口
    pub effective_date: String,
    pub customer: String,
    pub end_cusumer_id: String,
    pub date_activated: String,
    pub date_updated: String,
    pub date_shipped: String,
    pub account_id: String,
    #[serde(rename = "fixedIPAddress")]
    pub fixed_ip_address: String,
    pub sim_notes: String,
    #[serde(rename = "deviceID")]
    pub device_id: String,
    #[serde(rename = "modemID")]
    pub modem_id: String,
    #[serde(rename = "globalSIMType")]
    pub global_sim_type: String,
    // flow 接口
    pub timestamp: String,
    pub cycle_start_date: String,
    pub cycle_end_date: String,
    pub device_cycle_usage_in_zones: String,
    pub data_usage_unit: String,
    pub ctd_data_usage: f64,
    #[serde(rename = "ctdSMSUsage")]
    pub ctd_sms_usage: f64,
    pub ctd_voice_usage: f64,
    pub ctd_session_count: f64,
    pub overage_limit_reached: bool,
    pub overage_limit_override: String,
    // flowUsage 接口
    pub zone: String,
    pub rate_plan_version: String,
    pub data_usage: f64,
    #[serde(rename = "smsmousage")]
    pub sms_mo_usage: f64,
    #[serde(rename = "smsmtusage")]
    pub sms_mt_usage: f64,
    #[serde(rename = "voiceMOUsage")]
    pub voice_mo_usage: f64,
    #[serde(rename = "voiceMOUsageUnit")]
    pub voice_mo_usage_unit: String,
    #[serde(rename = "voiceMTUsage")]
    pub voice_mt_usage: f64,
    #[serde(rename = "voiceMTUsageUnit")]
    pub voice_mt_usage_unit: String,
    // 短信
    pub sms_message_id: i64,
}

impl ChinaUnicomClient {
    pub fn rest_request(&self, method: Method, url: String) -> Result<String, &str> {
        let client = match ClientBuilder::new()
            .gzip(true)
            .timeout(Duration::from_secs(10))
            .build() {
                Ok(c) => c,
                Err(_) => return Err("failed to build client"),
            };
        let url = Url::parse(&format!("{}{}", CHINA_UNICOM_REST_API_URL, url)).unwrap();
        let mut req = Request::new(method, url);
        let token = &format!("Basic {}", base64::encode(format!("{}:{}", self.username, self.rest_license).as_bytes()));
        req.headers_mut().insert("Authorization", token.parse().unwrap());
        req.headers_mut().insert("Content-Type", "application/json".parse().unwrap());
        req.headers_mut().insert("User-Agent", "rust client by kshih.com".parse().unwrap());
        let mut resp = client.execute(req).expect("failed to send request");
        // if let Ok(reply) = resp.json::<ChinaUnicomReply<'a>>() {
        //     println!("{:#?}", reply);
        // };
        // let reply: ChinaUnicomReply = resp.json().expect("not right");
        println!("{:?}, {:?}", resp, resp);
        Ok("结束".to_string())
    }
}

impl ChinaUnicomReply {
    fn to_card_status(&self) -> String {
        "to_card_status".to_string()
    }
    fn to_card_info(&self) -> String {
        "to_card_info".to_string()
    }
}

impl crate::client::CarrierClient for ChinaUnicomClient {
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