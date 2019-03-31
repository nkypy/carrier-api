use hashbrown::HashMap;
use lazy_static::lazy_static;

use crate::{CardInfo, CardStatus, Result};

lazy_static! {
    static ref STATUS_NAME_HASHMAP: HashMap<&'static str, &'static str> = {
        let m: HashMap<&'static str, &'static str> = [
            ("1", "正常"),
            ("2", "待激活"),
            ("3", "停机"),
            ("4", "销户"),
            ("8", "全停"),
            ("9", "全停"),
        ]
        .iter()
        .cloned()
        .collect();
        m
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardReply {
    pub status: String,
    pub message: String,
    pub result: Vec<CardReplyResult>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardReplyResult {
    pub status: String,
    pub message: String,
    pub iccid: String,
    pub msisdn: String,
    pub imsi: String,
    pub prod_id: String,
    pub prod_inst_id: String,
    pub prod_name: String,
    pub gprs_total: String,
    pub gprs_used: String,
    pub gprs_left: String,
    #[serde(rename = "GPRSSTATUS")]
    pub gprs_status: String,
    #[serde(rename = "IP")]
    pub ip: String,
    #[serde(rename = "APN")]
    pub apn: String,
    #[serde(rename = "RAT")]
    pub rat: String,
    pub open_time: String,
    pub apnname: String,
    pub total_gprs: String,
}

impl CardReply {
    pub fn to_card_status(&self) -> Result<CardStatus> {
        if self.status.as_str() != "0" {
            return Err(self.message.to_string())?;
        };
        let status_code: &str = &self.result[0].status;
        let status_name = match STATUS_NAME_HASHMAP.get(status_code) {
            Some(name) => name,
            None => "未知状态",
        };
        Ok(CardStatus {
            status_code: status_code.to_owned(),
            status_name: status_name.to_owned(),
            date_activated: "未知".to_string(),
        })
    }
}
