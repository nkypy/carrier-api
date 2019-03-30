use std::convert::From;
use std::str::FromStr;

use hashbrown::HashMap;
use lazy_static::lazy_static;
use serde_json;

use crate::{CardInfo, CardStatus, Result, STATUS_NAME_HASHMAP};

lazy_static! {
    static ref ERROR_HASHMAP: HashMap<&'static str, (&'static str, &'static str)> = {
        let m: HashMap<&'static str, (&'static str, &'static str)> = [
            ("10000001", ("30000001", "API 凭据无效。")),
            ("10000002", ("30000002", "账户 ID 缺失。")),
            ("10000003", ("30000003", "日期/时间缺失。")),
            ("10000004", ("30000004", "账户 ID 无效。")),
            ("10000005", ("30000005", "SIM 卡状态无效。")),
            ("10000006", ("30000006", "页面大小无效。")),
            ("10000007", ("30000007", "页面编号无效。")),
            (
                "10000008",
                (
                    "30000008",
                    "您的角色无权编辑运营商自定义字段。",
                ),
            ),
            (
                "10000009",
                (
                    "30000009",
                    "您的角色无权编辑账户自定义字段。",
                ),
            ),
            (
                "10000010",
                (
                    "30000010",
                    "您的角色无权编辑客户自定义字段。",
                ),
            ),
            (
                "10000011",
                ("30000011", "一个或多个必填字段缺失。"),
            ),
            ("10000012", ("30000012", "日期格式无效。")),
            ("10000013", ("30000013", "资费计划无效。")),
            ("10000014", ("30000014", "通信计划无效。")),
            ("10000015", ("30000015", "客户无效。")),
            (
                "10000016",
                ("30000016", "overageLimitOverride 值无效。"),
            ),
            ("10000017", ("30000017", "messageEncoding 无效。")),
            ("10000018", ("30000018", "dataCoding 无效。")),
            ("10000019", ("30000019", "validityPeriod 无效。")),
            ("10000020", ("30000020", "消息包含过多字符。")),
            ("10000021", ("30000021", "ICCID 无效。")),
            ("10000022", ("30000022", "fromDate 缺失。")),
            (
                "10000023",
                (
                    "30000023",
                    "请求中的 JSON 格式不正确。请确保逗号、冒号、括号等格式正确。",
                ),
            ),
            ("10000024", ("30000024", "API 版本编号无效。")),
            ("10000025", ("30000025", "找不到设备 ID。")),
            ("10000026", ("30000026", "找不到调制解调器 ID。")),
            (
                "10000027",
                ("30000027", "toDate 必须在 fromDate 之后。"),
            ),
            (
                "10000028",
                (
                    "30000028",
                    "请求包含一个或多个无法识别的参数。",
                ),
            ),
            (
                "10000029",
                (
                    "30000029",
                    "此设备无法重新变回“预激活”状态。",
                ),
            ),
            (
                "10000030",
                ("30000030", "使用的角色无法访问此 API。"),
            ),
            ("10000031", ("30000031", "区域无效。")),
            (
                "10000032",
                (
                    "30000032",
                    "cycleStartDate 必须指定最后三个计费周期(包括当前周期)之一。",
                ),
            ),
            (
                "10000049",
                ("30000049", "daysOfHistory 必须小于等于 365。"),
            ),
            ("20000001", ("30000101", "找不到指定 ICCID。")),
            ("20000002", ("30000102", "找不到指定短消息 ID。")),
            ("30000001", ("30000201", "未知服务器错误。")),
            (
                "30000002",
                (
                    "30000202",
                    "Control Center 无法将消息提交至 SMSC。",
                ),
            ),
            (
                "40000029",
                (
                    "30000329",
                    "接口请求超过限制次数，请稍后再试。",
                ),
            ),
        ]
        .iter()
        .cloned()
        .collect();
        m
    };
}

// SOAP 请求格式
pub struct RequestEnvelope {}

impl RequestEnvelope {
    pub fn new(
        username: &str,
        password: &str,
        license: &str,
        method: &str,
        iccids: Vec<&str>,
    ) -> String {
        let iccid_vec: Vec<String> = iccids
            .iter()
            .map(|x| format!("<jws:iccid>{}</jws:iccid>", x))
            .collect();
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<env:Envelope xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:jws="http://api.jasperwireless.com/ws/schema" xmlns:env="http://schemas.xmlsoap.org/soap/envelope/">
<env:Header>
<wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
<wsse:UsernameToken wsu:Id="UsernameToken-1" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
<wsse:Username>{}</wsse:Username>
<wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordText">{}</wsse:Password>
</wsse:UsernameToken>
</wsse:Security>
</env:Header>
<env:Body>
<jws:{}Request>
<jws:messageId></jws:messageId>
<jws:version></jws:version>
<jws:licenseKey>{}</jws:licenseKey>
<jws:iccids>{}</jws:iccids>
</jws:{}Request>
</env:Body>
</env:Envelope>"#, username, password,method, license, iccid_vec.join(""), method).replace('\n', "");
        text
    }
}

// 发送短信请求格式
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsRequest {
    pub message_text: String,
    pub message_encoding: String,
}

// 返回信息格式
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct CardReply {
    // 错误
    pub error_code: String,
    // pub error_message: String, // 错误详情，无需解析
    // 基本信息
    pub iccid: String,
    pub imsi: String,
    pub msisdn: String,
    pub imei: String,
    pub status: String,
    pub rate_plan: String,
    pub communication_plan: String,
    // Info 接口
    pub effective_date: String,
    pub customer: Option<String>,
    pub end_cusumer_id: Option<String>,
    pub date_activated: String,
    pub date_updated: String,
    pub date_shipped: String,
    pub account_id: String,
    #[serde(rename = "fixedIPAddress")]
    pub fixed_ip_address: Option<String>,
    pub sim_notes: String,
    pub euiccid: Option<String>,
    #[serde(rename = "deviceID")]
    pub device_id: Option<String>,
    #[serde(rename = "modemID")]
    pub modem_id: Option<String>,
    #[serde(rename = "globalSIMType")]
    pub global_sim_type: String,
    // flow 接口
    pub timestamp: String,
    pub cycle_start_date: String,
    pub cycle_end_date: String,
    pub device_cycle_usage_in_zones: String,
    pub data_usage_unit: String,
    pub ctd_data_usage: Option<f64>,
    #[serde(rename = "ctdSMSUsage")]
    pub ctd_sms_usage: Option<f64>,
    pub ctd_voice_usage: Option<f64>,
    pub ctd_session_count: Option<f64>,
    pub overage_limit_reached: Option<bool>,
    pub overage_limit_override: String,
    // flowUsage 接口
    pub zone: String,
    pub rate_plan_version: String,
    pub data_usage: Option<f64>,
    #[serde(rename = "smsmousage")]
    pub sms_mo_usage: Option<f64>,
    #[serde(rename = "smsmtusage")]
    pub sms_mt_usage: Option<f64>,
    #[serde(rename = "voiceMOUsage")]
    pub voice_mo_usage: Option<f64>,
    #[serde(rename = "voiceMOUsageUnit")]
    pub voice_mo_usage_unit: String,
    #[serde(rename = "voiceMTUsage")]
    pub voice_mt_usage: Option<f64>,
    #[serde(rename = "voiceMTUsageUnit")]
    pub voice_mt_usage_unit: String,
    // 短信
    pub sms_message_id: Option<i64>,
}

impl CardReply {
    pub fn is_err(self) -> Result<Self> {
        if self.error_code.as_str() != "" {
            match ERROR_HASHMAP.get(self.error_code.as_str()) {
                Some(t) => Err(t.to_owned())?,
                None => Err(("30999999", "中国联通未知错误。"))?,
            };
        };
        Ok(self)
    }
}

impl FromStr for CardReply {
    type Err = crate::errors::Error;
    fn from_str(s: &str) -> Result<Self> {
        let r: Self = serde_json::from_str(s)?;
        Ok(r.is_err()?)
    }
}

impl From<CardReply> for CardStatus {
    fn from(s: CardReply) -> Self {
        let status_name = match STATUS_NAME_HASHMAP
            .get("china_unicom")
            .unwrap()
            .get(s.status.as_str())
        {
            Some(name) => name,
            None => "未知状态",
        };
        CardStatus {
            status_code: s.status,
            status_name: status_name.to_owned(),
            date_activated: s.date_activated,
        }
    }
}

impl From<CardReply> for CardInfo {
    fn from(s: CardReply) -> Self {
        CardInfo {
            iccid: s.iccid,
            imsi: s.imsi,
            msisdn: s.msisdn,
            imei: s.imei,
            region_name: "".to_owned(),
            customer_name: s.account_id,
            brand: "".to_owned(),
        }
    }
}
