use crate::{CardStatus, CardInfo};

// 发送短信请求格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsRequest<'a> {
	pub message_text: &'a str,
	pub message_encoding: &'a str,
}

// 返回数据格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardReply<'a> {
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

impl<'a> CardReply<'a> {
    fn to_card_status(&self) -> Result<CardStatus, &'a str> {
        Err("to_card_status")
    }
    fn to_card_info(&self) -> Result<CardInfo, &'a str> {
        Err("to_card_info")
    }
}
