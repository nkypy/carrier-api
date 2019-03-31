use hashbrown::HashMap;
use lazy_static::lazy_static;

use crate::{CardInfo, CardStatus, Result};

lazy_static! {
    static ref STATUS_NAME_HASHMAP: HashMap<&'static str, &'static str> = {
        let m: HashMap<&'static str, &'static str> = [
            ("test", "测试期"),
            ("silent", "沉默期"),
            ("inventory", "库存期"),
            ("normal", "正使用"),
            ("stop", "停机"),
            ("preclose", "销户"),
            ("bespeakClose", "预约销户"),
            ("others", "其他"),
        ]
        .iter()
        .cloned()
        .collect();
        m
    };
}

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
