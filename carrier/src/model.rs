// 标准状态查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardStatus {
    pub status_code: String,
    pub status_name: String,
    pub date_activated: String,
}

// 标准信息查询返回
#[derive(Debug, Serialize, Deserialize)]
pub struct CardInfo {
    pub iccid: String,
    pub imsi: String,
    pub msisdn: String,
    pub imei: String,
    pub region_name: String,
    pub customer_name: String,
    pub brand: String,
}
