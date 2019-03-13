use crate::{CardStatus, CardInfo};

// 卡号查询返回
#[derive(Debug, Serialize, Deserialize)]
struct CardMsisdnReply<'a> {
    #[serde(rename = "RESULT")]
	result: &'a str,
    #[serde(rename = "SMSG")]
	msisdn: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct CardStatusReplyProductInfo<'a> {
    #[serde(rename = "productMainStatusCd")]
    status_code: &'a str,
    #[serde(rename = "productMainStatusName")]
    status_name: &'a str,
}

// 状态查询返回
#[derive(Debug, Serialize, Deserialize)]
struct CardStatusReply<'a> {
    #[serde(rename = "resultCode")]
    result_code: &'a str,
    #[serde(rename = "resultMsg")]
    result_message: &'a str,
    #[serde(rename = "GROUP_TRANSACTIONID")]
    transaction_id: &'a str,
    #[serde(rename = "number")]
    msisdn: &'a str,
    #[serde(rename = "servCreateDate")]
    date_created: &'a str,
    #[serde(rename = "productInfo")]
    infos: Vec<CardStatusReplyProductInfo<'a>>
}

impl<'a> CardStatusReply<'a> {
    fn to_card_status(&self) -> Result<CardStatus, &'a str> {
        match self.result_code {
            "0" => match self.infos.len() {
                0 => Err("长度不对"),
                1 => Ok(CardStatus{
                    status_code: self.infos[0].status_code,
                    status_name: self.infos[0].status_name,
                    date_created: self.date_created}),
                _ => {
                    for v in self.infos.iter() {
                        if v.status_code != "6" {
                            return Ok(CardStatus{
                                status_code: v.status_code,
                                status_name: v.status_name,
                                date_created: self.date_created});
                        }
                    }
                    Err("没有数据")
                }
            }
            _ => Err("错误"),
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
    fn to_card_info(&self) -> Result<CardInfo, &'a str> {
        match self.result_code {
            "0" => Ok(CardInfo{
                iccid: "", imsi: "", msisdn: self.result.infos.msisdn, imei: "",
                region_name: self.result.infos.region_name,
                customer_name: self.result.infos.customer_name, brand: ""}),
            _ => Err("错误"),
        }
    }
}
