use std::convert::From;
use std::str::FromStr;

use hashbrown::HashMap;
use lazy_static::lazy_static;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::{CardInfo, CardStatus, Result};

lazy_static! {
    static ref ERROR_HASHMAP: HashMap<&'static str, (&'static str, &'static str)> = {
        let m: HashMap<&'static str, (&'static str, &'static str)> = [
            (
                "-1",
                ("20000001", "用户名或者密码错误，无权限。"),
            ),
            ("-2", ("20000002", "必传参数缺失。")),
            ("-3", ("20000003", "sign参数值错误。")),
            ("-4", ("20000004", "iccid号码错误或不存在。")),
            (
                "-5",
                (
                    "20000005",
                    "已经订购该产品，请不要重复订购。",
                ),
            ),
            (
                "-7",
                (
                    "20000007",
                    "该号码已有业务正在处理，请稍后下发。",
                ),
            ),
            (
                "101004",
                ("20000014", "请求过于频繁，请稍后再试。"),
            ),
        ]
        .iter()
        .cloned()
        .collect();
        m
    };
    static ref STATUS_NAME_HASHMAP: HashMap<&'static str, &'static str> = {
        let m: HashMap<&'static str, &'static str> = [
            ("1", "在用"),
            ("2", "用户报停"),
            ("3", "用户拆机"),
            ("5", "欠停(双向)"),
            ("6", "欠停(单向)"),
            ("7", "违章停机"),
            ("8", "挂失"),
            ("19", "活卡待激活"),
            ("120000", "停机"),
            ("100001", "已激活"),
            ("140003", "未激活"),
            ("150001", "未实名制违规停机"),
            ("99999", "销户"),
        ]
        .iter()
        .cloned()
        .collect();
        m
    };
}

// 卡号查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardMsisdnReply {
    #[serde(rename = "RESULT")]
    pub result: String,
    #[serde(rename = "SMSG")]
    pub msisdn: String,
}

fn str_to_error_tuple(s: &str) -> (&str, &str) {
    let v: Vec<&str> = s.split('：').collect();
    match ERROR_HASHMAP.get(v[0]) {
        Some(e) => e.to_owned(),
        None => ("20999999", s),
    }
}

impl FromStr for CardMsisdnReply {
    type Err = crate::errors::Error;
    fn from_str(s: &str) -> Result<Self> {
        let r: Self = serde_json::from_str(s).map_err(|_| str_to_error_tuple(s))?;
        Ok(r)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardStatusReplyProductInfo {
    #[serde(rename = "productMainStatusCd")]
    pub status_code: String,
    #[serde(rename = "productMainStatusName")]
    pub status_name: String,
}

// 状态查询返回
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CardStatusReply {
    #[serde(rename = "resultCode")]
    pub result_code: String,
    #[serde(rename = "resultMsg")]
    pub result_message: String,
    #[serde(rename = "GROUP_TRANSACTIONID")]
    pub transaction_id: String,
    #[serde(rename = "number")]
    pub msisdn: String,
    #[serde(rename = "servCreateDate")]
    pub date_created: String,
    #[serde(rename = "productInfo")]
    pub infos: Vec<CardStatusReplyProductInfo>,
}

impl FromStr for CardStatusReply {
    type Err = crate::errors::Error;
    fn from_str(s: &str) -> Result<Self> {
        let r: Self = serde_json::from_str(s).map_err(|e| {
            dbg!(e);
            str_to_error_tuple(s)
        })?;
        if r.result_code.as_str() != "0" {
            match ERROR_HASHMAP.get(r.result_code.as_str()) {
                Some(e) => Err(e.to_owned())?,
                None => Err(("20999999", s))?,
            };
        };
        Ok(r)
    }
}

impl From<CardStatusReply> for CardStatus {
    fn from(s: CardStatusReply) -> Self {
        for v in s.infos.iter() {
            if v.status_code != "6" {
                return CardStatus {
                    status_code: v.status_code.to_owned(),
                    status_name: v.status_name.to_owned(),
                    date_activated: s.date_created.to_owned(),
                };
            };
        }
        CardStatus {
            status_code: "未知".to_owned(),
            status_name: "未知".to_owned(),
            date_activated: "未知".to_owned(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename = "prodInfos")]
pub struct CardInfoReplyResult {
    #[serde(rename = "commonRegionName")]
    pub region_name: String,
    #[serde(rename = "custName")]
    pub customer_name: String,
    #[serde(rename = "phoneNum")]
    pub msisdn: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "prodStatusName")]
    pub product_status_name: String,
    #[serde(rename = "stopFlag")]
    pub stop_flag: String,
}

// 信息查询返回
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename = "SvcCont")]
pub struct CardInfoReply {
    #[serde(rename = "resultCode")]
    pub result_code: String,
    #[serde(rename = "resultMsg")]
    pub result_message: String,
    #[serde(rename = "GROUP_TRANSACTIONID")]
    pub transaction_id: String,
    pub result: CardInfoReplyResult,
}

impl FromStr for CardInfoReply {
    type Err = crate::errors::Error;
    fn from_str(s: &str) -> Result<Self> {
        // xml 手动读取
        let mut r: Self = Default::default();
        r.result_code = "0".to_owned();
        let mut reader = Reader::from_str(s);
        reader.trim_text(true);
        // 循环
        let mut buf = Vec::new();
        let mut m: HashMap<String, String> = HashMap::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let n = e.unescape_and_decode(&reader).expect("Error!");
                    let v = match reader.read_text(e.name(), &mut Vec::new()) {
                        Ok(t) => t,
                        Err(_) => "".to_owned(),
                    };
                    m.insert(n, v);
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        dbg!(&m);
        // 写入数据
        r.result_code = match m.get("resultCode") {
            Some(t) => t.to_owned(),
            None => "未知".to_owned(),
        };
        r.result_message = match m.get("resultMsg") {
            Some(t) => t.to_owned(),
            None => "未知".to_owned(),
        };
        r.transaction_id = match m.get("GROUP_TRANSACTIONID") {
            Some(t) => t.to_owned(),
            None => "未知".to_owned(),
        };
        r.result.msisdn = match m.get("phoneNum") {
            Some(t) => t.to_owned(),
            None => "未知".to_owned(),
        };
        r.result.region_name = match m.get("commonRegionName") {
            Some(t) => t.to_owned(),
            None => "未知".to_owned(),
        };
        r.result.customer_name = match m.get("custName") {
            Some(t) => t.to_owned(),
            None => "未知".to_owned(),
        };
        //写入结束
        if r.result_code.as_str() != "0" {
            match ERROR_HASHMAP.get(r.result_code.as_str()) {
                Some(e) => Err(e.to_owned())?,
                None => Err(("20999999", s))?,
            };
        };
        Ok(r)
    }
}

impl From<CardInfoReply> for CardInfo {
    fn from(s: CardInfoReply) -> Self {
        CardInfo {
            iccid: "".to_owned(),
            imsi: "".to_owned(),
            msisdn: s.result.msisdn,
            imei: "".to_owned(),
            region_name: s.result.region_name,
            customer_name: s.result.customer_name,
            brand: "".to_owned(),
        }
    }
}
