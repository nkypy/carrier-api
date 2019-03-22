use {lazy_static::lazy_static, std::collections::HashMap};

use crate::{CardInfo, CardStatus, Result};

lazy_static! {
    static ref STATUS_HASHMAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("ACTIVATION_READY", "可激活");
        m.insert("ACTIVATED", "已激活");
        m.insert("DEACTIVATED", "已停用");
        m.insert("RETIRED", "已失效");
        m
    };
}

// 发送短信请求格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsRequest {
    pub message_text: String,
    pub message_encoding: String,
}

// 返回数据格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardReply {
    // 错误
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    // 基本信息
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub msisdn: Option<String>,
    pub imei: Option<String>,
    pub status: Option<String>,
    pub rate_plan: Option<String>,
    pub communication_plan: Option<String>,
    // info 接口
    pub effective_date: Option<String>,
    pub customer: Option<String>,
    pub end_cusumer_id: Option<String>,
    pub date_activated: Option<String>,
    pub date_updated: Option<String>,
    pub date_shipped: Option<String>,
    pub account_id: Option<String>,
    #[serde(rename = "fixedIPAddress")]
    pub fixed_ip_address: Option<String>,
    pub sim_notes: Option<String>,
    #[serde(rename = "deviceID")]
    pub device_id: Option<String>,
    #[serde(rename = "modemID")]
    pub modem_id: Option<String>,
    #[serde(rename = "globalSIMType")]
    pub global_sim_type: Option<String>,
    // flow 接口
    pub timestamp: Option<String>,
    pub cycle_start_date: Option<String>,
    pub cycle_end_date: Option<String>,
    pub device_cycle_usage_in_zones: Option<String>,
    pub data_usage_unit: Option<String>,
    pub ctd_data_usage: Option<f64>,
    #[serde(rename = "ctdSMSUsage")]
    pub ctd_sms_usage: Option<f64>,
    pub ctd_voice_usage: Option<f64>,
    pub ctd_session_count: Option<f64>,
    pub overage_limit_reached: Option<bool>,
    pub overage_limit_override: Option<String>,
    // flowUsage 接口
    pub zone: Option<String>,
    pub rate_plan_version: Option<String>,
    pub data_usage: Option<f64>,
    #[serde(rename = "smsmousage")]
    pub sms_mo_usage: Option<f64>,
    #[serde(rename = "smsmtusage")]
    pub sms_mt_usage: Option<f64>,
    #[serde(rename = "voiceMOUsage")]
    pub voice_mo_usage: Option<f64>,
    #[serde(rename = "voiceMOUsageUnit")]
    pub voice_mo_usage_unit: Option<String>,
    #[serde(rename = "voiceMTUsage")]
    pub voice_mt_usage: Option<f64>,
    #[serde(rename = "voiceMTUsageUnit")]
    pub voice_mt_usage_unit: Option<String>,
    // 短信
    pub sms_message_id: Option<i64>,
}

impl CardReply {
    pub fn to_card_status(&self) -> Result<CardStatus> {
        if let (Some(_code), Some(msg)) = (&self.error_code, &self.error_message) {
            return Err(msg.to_owned());
        }
        if let (Some(code), Some(date)) = (&self.status, &self.date_activated) {
            let status_code: &str = &code.to_string();
            let status_name = match STATUS_HASHMAP.get(status_code) {
                Some(name) => name,
                None => "未知状态",
            };
            return Ok(CardStatus {
                status_code: code.to_owned(),
                status_name: status_name.to_string(),
                date_activated: date.to_owned(),
            });
        }
        Err("数据解析问题".to_owned())
    }

    pub fn to_card_info(&self) -> Result<CardInfo> {
        Err("to_card_info".to_string())
    }
}
