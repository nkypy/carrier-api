use crate::{CardInfo, CardStatus, Result};

// 广东移动返回
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CardReply<'a> {
    code: &'a str,
    error: &'a str,
    data: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CardReplyContent<'a> {
    code: &'a str,
    desc: &'a str,
    order_no: &'a str,
    status: &'a str,
    status_time: &'a str,
    iccid: &'a str,
    msisdn: &'a str,
    imsi: &'a str,
    imei: &'a str,
    open_time: &'a str,
    low_power_mode: &'a str,
    main_prod_code: &'a str,
}
