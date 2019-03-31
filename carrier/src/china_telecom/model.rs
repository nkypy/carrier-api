use serde_json;

use crate::{CardInfo, CardStatus, Result};

// 卡号查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardMsisdnReply {
    #[serde(rename = "RESULT")]
    pub result: String,
    #[serde(rename = "SMSG")]
    pub msisdn: String,
}

impl CardMsisdnReply {
    pub fn from_str(s: &str) -> Result<String> {
        let r: Self = serde_json::from_str(s)?;
        if r.result != "0" {
            Err(r.result)?
        }
        Ok(r.msisdn)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardStatusReplyProductInfo<'a> {
    #[serde(rename = "productMainStatusCd")]
    pub status_code: &'a str,
    #[serde(rename = "productMainStatusName")]
    pub status_name: &'a str,
}

// 状态查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardStatusReply<'a> {
    #[serde(rename = "resultCode")]
    pub result_code: &'a str,
    #[serde(rename = "resultMsg")]
    pub result_message: &'a str,
    #[serde(rename = "GROUP_TRANSACTIONID")]
    pub transaction_id: &'a str,
    #[serde(rename = "number")]
    pub msisdn: &'a str,
    #[serde(rename = "servCreateDate")]
    pub date_created: &'a str,
    #[serde(rename = "productInfo")]
    pub infos: Vec<CardStatusReplyProductInfo<'a>>,
}

impl<'a> CardStatusReply<'a> {
    pub fn to_card_status(&self) -> Result<CardStatus> {
        match self.result_code {
            "0" => match self.infos.len() {
                0 => Err("长度不对".to_owned())?,
                1 => Ok(CardStatus {
                    status_code: self.infos[0].status_code.to_owned(),
                    status_name: self.infos[0].status_name.to_owned(),
                    date_activated: self.date_created.to_owned(),
                }),
                _ => {
                    for v in self.infos.iter() {
                        if v.status_code != "6" {
                            return Ok(CardStatus {
                                status_code: v.status_code.to_owned(),
                                status_name: v.status_name.to_owned(),
                                date_activated: self.date_created.to_owned(),
                            });
                        }
                    }
                    Err("没有数据".to_owned())?
                }
            },
            _ => Err("错误".to_owned())?,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CardInfoReplyInfo<'a> {
    #[serde(rename = "commonRegionName")]
    region_name: &'a str,
    #[serde(rename = "custName")]
    customer_name: &'a str,
    #[serde(rename = "phoneNum")]
    msisdn: &'a str,
    #[serde(rename = "productName")]
    product_name: &'a str,
    #[serde(rename = "prodStatusName")]
    product_status_name: &'a str,
    #[serde(rename = "stopFlag")]
    stop_flag: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct CardInfoReplyResult<'a> {
    #[serde(borrow, rename = "prodInfos")]
    infos: CardInfoReplyInfo<'a>,
}

// 信息查询返回
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "SvcCont")]
struct CardInfoReply<'a> {
    #[serde(rename = "resultCode")]
    result_code: &'a str,
    #[serde(rename = "resultMsg")]
    result_message: &'a str,
    #[serde(rename = "GROUP_TRANSACTIONID")]
    transaction_id: &'a str,
    #[serde(rename = "result")]
    result: CardInfoReplyResult<'a>,
}

impl<'a> CardInfoReply<'a> {
    fn to_card_info(&self) -> Result<CardInfo> {
        match self.result_code {
            "0" => Ok(CardInfo {
                iccid: "".to_owned(),
                imsi: "".to_owned(),
                msisdn: self.result.infos.msisdn.to_owned(),
                imei: "".to_owned(),
                region_name: self.result.infos.region_name.to_owned(),
                customer_name: self.result.infos.customer_name.to_owned(),
                brand: "".to_owned(),
            }),
            _ => Err("错误".to_owned())?,
        }
    }
}
